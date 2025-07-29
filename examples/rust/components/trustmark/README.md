# WebGPU with Trustmark Example

This is a Rust WebAssembly component that demonstrates using WebGPU for Trustmark image processing. The component receives image data via HTTP POST requests and processes them using the Trustmark library with WebGPU acceleration.

## Prerequisites

- `cargo` 1.82
- [`wash`](https://wasmcloud.com/docs/installation) 0.36.1
- `wasmtime` >=25.0.0 (if running with wasmtime)
- WebGPU-capable system (for WebGPU functionality)

## Setup

Download the required Trustmark models using the provided script:

```bash
# Download all model files automatically
./download_models.sh
```

This script will:
- Create the `models/` directory if it doesn't exist
- Download all 8 ONNX model files from `https://cc-assets.netlify.app/watermarking/trustmark-models`
- Provide progress feedback during download
- Show a summary of downloaded files

Alternatively, you can download the models manually:

```bash
# Create models directory if it doesn't exist
mkdir -p models

# Download all models from the Trustmark assets
curl -L https://cc-assets.netlify.app/watermarking/trustmark-models/encoder_B.onnx -o models/encoder_B.onnx
curl -L https://cc-assets.netlify.app/watermarking/trustmark-models/encoder_C.onnx -o models/encoder_C.onnx
curl -L https://cc-assets.netlify.app/watermarking/trustmark-models/encoder_P.onnx -o models/encoder_P.onnx
curl -L https://cc-assets.netlify.app/watermarking/trustmark-models/encoder_Q.onnx -o models/encoder_Q.onnx
curl -L https://cc-assets.netlify.app/watermarking/trustmark-models/decoder_B.onnx -o models/decoder_B.onnx
curl -L https://cc-assets.netlify.app/watermarking/trustmark-models/decoder_C.onnx -o models/decoder_C.onnx
curl -L https://cc-assets.netlify.app/watermarking/trustmark-models/decoder_P.onnx -o models/decoder_P.onnx
curl -L https://cc-assets.netlify.app/watermarking/trustmark-models/decoder_Q.onnx -o models/decoder_Q.onnx
```

## Building

```bash
wash build
```

**Note**: This component requires the Trustmark library which uses WebGPU for acceleration. The component is built with WebAssembly Component Model tooling.

## Running with wasmtime

You must have wasmtime >=25.0.0 for this to work. Make sure to follow the build step above first.

```bash
wasmtime serve -Scommon ./build/http_hello_world_s.wasm
```

## Running with wasmCloud

```shell
wash dev
```

## Testing

Send an image for Trustmark processing:

```shell
curl -X POST http://127.0.0.1:8000/ \
  -H "Content-Type: image/png" \
  -H "Content-Length: $(stat -f%z your_image.png)" \
  --data-binary "@your_image.png"
```

The component accepts various image formats:
- `image/jpeg` or `image/jpg`
- `image/png`
- `image/gif`
- `image/bmp`
- `image/tiff`

## WebGPU Integration

This example demonstrates:
- WebGPU integration for GPU-accelerated image processing
- Trustmark watermarking using WebGPU
- HTTP-based image input/output
- WebAssembly Component Model compatibility

## Adding Capabilities

To learn how to extend this example with additional capabilities, see the [Adding Capabilities](https://wasmcloud.com/docs/tour/adding-capabilities?lang=rust) section of the wasmCloud documentation.
