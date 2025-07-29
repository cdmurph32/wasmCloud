#!/bin/bash

# Download Trustmark model files from cc-assets.netlify.app
# This script downloads all the required ONNX model files for the Trustmark component

set -e  # Exit on any error

# Base URL for the models
BASE_URL="https://cc-assets.netlify.app/watermarking/trustmark-models"

# Create models directory if it doesn't exist
mkdir -p models

# List of model files to download
MODELS=(
    "encoder_B.onnx"
    "encoder_C.onnx"
    "encoder_P.onnx"
    "encoder_Q.onnx"
    "decoder_B.onnx"
    "decoder_C.onnx"
    "decoder_P.onnx"
    "decoder_Q.onnx"
)

echo "Downloading Trustmark model files..."

# Download each model file
for model in "${MODELS[@]}"; do
    echo "Downloading $model..."
    if curl -L -o "models/$model" "$BASE_URL/$model"; then
        echo "✓ Successfully downloaded $model"
    else
        echo "✗ Failed to download $model"
        exit 1
    fi
done

echo ""
echo "All model files downloaded successfully!"
echo "Files downloaded to: $(pwd)/models/"
echo ""
echo "Downloaded files:"
ls -lh models/*.onnx 