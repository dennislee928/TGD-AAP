// inference_handler.rs — tonic gRPC service implementation.

use tonic::{Request, Response, Status};
use tracing::info;

use crate::generated::{
    inference_service_server::InferenceService, PredictRequest, PredictResponse,
};

/// Concrete implementation of the InferenceService gRPC contract.
pub struct InferenceHandler;

impl InferenceHandler {
    pub fn new() -> Self {
        Self
    }
}

#[tonic::async_trait]
impl InferenceService for InferenceHandler {
    async fn predict(
        &self,
        request: Request<PredictRequest>,
    ) -> Result<Response<PredictResponse>, Status> {
        let req = request.into_inner();
        info!("Predict called: request_id={}", req.request_id);

        // TODO: load model weights and run inference via quantum_bridge.
        let mock_confidence = 0.42_f32;
        let alert_triggered = mock_confidence >= 0.8;

        let reply = PredictResponse {
            request_id: req.request_id,
            predictions: vec![mock_confidence],
            confidence: mock_confidence,
            alert_triggered,
            message: format!("Inference complete. confidence={:.4}", mock_confidence),
        };

        Ok(Response::new(reply))
    }

    type PredictStreamStream =
        tokio_stream::wrappers::ReceiverStream<Result<PredictResponse, Status>>;

    async fn predict_stream(
        &self,
        request: Request<PredictRequest>,
    ) -> Result<Response<Self::PredictStreamStream>, Status> {
        let req = request.into_inner();
        info!("PredictStream called: request_id={}", req.request_id);

        let (tx, rx) = tokio::sync::mpsc::channel(16);

        tokio::spawn(async move {
            for i in 0..5u32 {
                let _ = tx
                    .send(Ok(PredictResponse {
                        request_id: req.request_id.clone(),
                        predictions: vec![i as f32 * 0.1],
                        confidence: i as f32 * 0.1,
                        alert_triggered: false,
                        message: format!("stream chunk {}", i),
                    }))
                    .await;
            }
        });

        Ok(Response::new(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }
}
