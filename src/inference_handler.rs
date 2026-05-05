// inference_handler.rs — tonic gRPC service implementation.

use tonic::{Request, Response, Status};
use tracing::{info, warn};

use crate::generated::{
    inference_service_server::InferenceService, PredictRequest, PredictResponse,
};
use crate::quantum_bridge;

/// Concrete implementation of the InferenceService gRPC contract.
pub struct InferenceHandler;

impl InferenceHandler {
    pub fn new() -> Self {
        Self
    }

    fn authorize(request: &Request<PredictRequest>) -> Result<(), Status> {
        let expected = std::env::var("GRPC_EXPECTED_TOKEN")
            .ok()
            .map(|v| v.trim().to_string())
            .unwrap_or_default();
        if expected.is_empty() {
            // Backward-compatible default: if no expected token is configured,
            // the service remains open.
            return Ok(());
        }

        let auth_header_token = request
            .metadata()
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| {
                let (scheme, token) = v.split_once(' ')?;
                if scheme.eq_ignore_ascii_case("bearer") {
                    Some(token.trim().to_string())
                } else {
                    None
                }
            });

        let legacy_body_token = request.get_ref().metadata.get("auth_token").cloned();
        let provided = auth_header_token.or(legacy_body_token).unwrap_or_default();

        if provided == expected {
            Ok(())
        } else {
            warn!("Unauthenticated gRPC request rejected");
            Err(Status::unauthenticated(
                "invalid or missing authorization bearer token",
            ))
        }
    }
}

#[tonic::async_trait]
impl InferenceService for InferenceHandler {
    async fn predict(
        &self,
        request: Request<PredictRequest>,
    ) -> Result<Response<PredictResponse>, Status> {
        Self::authorize(&request)?;
        let req = request.into_inner();
        info!("Predict called: request_id={}", req.request_id);

        let scored = quantum_bridge::score_features(&req.features).await;
        let alert_triggered = scored.confidence >= 0.8;
        let mode = if scored.used_quantum {
            "quantum"
        } else {
            "fallback"
        };

        let reply = PredictResponse {
            request_id: req.request_id,
            predictions: vec![scored.prediction],
            confidence: scored.confidence,
            alert_triggered,
            message: format!(
                "Inference complete via {} path. prediction={:.4}, confidence={:.4}",
                mode, scored.prediction, scored.confidence
            ),
        };

        Ok(Response::new(reply))
    }

    type PredictStreamStream =
        tokio_stream::wrappers::ReceiverStream<Result<PredictResponse, Status>>;

    async fn predict_stream(
        &self,
        request: Request<PredictRequest>,
    ) -> Result<Response<Self::PredictStreamStream>, Status> {
        Self::authorize(&request)?;
        let req = request.into_inner();
        info!("PredictStream called: request_id={}", req.request_id);

        let (tx, rx) = tokio::sync::mpsc::channel(16);

        tokio::spawn(async move {
            for i in 0..5u32 {
                let shifted_features: Vec<f32> = req
                    .features
                    .iter()
                    .map(|v| *v + (i as f32 * 0.01_f32))
                    .collect();
                let scored = quantum_bridge::score_features(&shifted_features).await;
                let _ = tx
                    .send(Ok(PredictResponse {
                        request_id: req.request_id.clone(),
                        predictions: vec![scored.prediction],
                        confidence: scored.confidence,
                        alert_triggered: scored.confidence >= 0.8,
                        message: if scored.used_quantum {
                            format!("stream chunk {} (quantum)", i)
                        } else {
                            format!("stream chunk {} (fallback)", i)
                        },
                    }))
                    .await;
            }
        });

        Ok(Response::new(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }
}
