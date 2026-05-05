# Models Directory

This directory stores model configuration files, quantization scripts, and
any locally cached model weights used by the TGD-AAP inference pipeline.

## Structure

```
models/
├── config.json          # Model architecture and hyperparameter configuration
├── quantize.py          # Post-training quantization helper script
└── weights/             # Downloaded / fine-tuned model weight files (gitignored)
```

## Notes

- Large weight files (`.bin`, `.safetensors`, `.pt`) are excluded from git via
  `.gitignore`. They are managed on Hugging Face Model Hub.
- Use `HF_TOKEN` environment variable when pulling private model repositories.
