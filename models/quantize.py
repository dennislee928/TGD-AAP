"""
quantize.py — Post-training quantization script for TGD-AAP models.

Downloads a model from Hugging Face Model Hub, applies INT8 quantization,
and re-uploads the optimized weights back to the hub.

Usage:
    HF_TOKEN=<token> python models/quantize.py --repo <hf-repo-id>
"""

import argparse
import os


def main() -> None:
    parser = argparse.ArgumentParser(description="Quantize a Hugging Face model to INT8")
    parser.add_argument("--repo", required=True, help="Hugging Face model repository ID")
    parser.add_argument("--output-dir", default="models/weights", help="Local output directory")
    args = parser.parse_args()

    hf_token = os.environ.get("HF_TOKEN")
    if not hf_token:
        raise EnvironmentError("HF_TOKEN environment variable is required")

    print(f"[quantize] Downloading model from {args.repo}")
    # TODO: implement download via huggingface_hub

    print("[quantize] Applying INT8 quantization")
    # TODO: implement quantization via bitsandbytes or ONNX Runtime

    print(f"[quantize] Saving quantized weights to {args.output_dir}")
    os.makedirs(args.output_dir, exist_ok=True)
    # TODO: save weights

    print("[quantize] Done")


if __name__ == "__main__":
    main()
