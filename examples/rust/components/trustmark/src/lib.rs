use trustmark::{Trustmark, Variant, Version};
use wasmcloud_component::http;

struct Component;

http::export!(Component);

const MODEL: &[u8] = include_bytes!("../models/encoder_B.onnx");

impl http::Server for Component {
    fn handle(
        request: http::IncomingRequest,
    ) -> http::Result<http::Response<impl http::OutgoingBody>> {
        let content_length = match request
            .headers()
            .get("Content-Length")
            .and_then(|hv| hv.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok())
        {
            Some(len) => len,
            None => {
                return Err(http::ErrorCode::HttpRequestLengthRequired);
            }
        };
        // Determine image format from Content-Type header
        let format = match request
            .headers()
            .get("Content-Type")
            .and_then(|hv| hv.to_str().ok())
        {
            Some("image/jpeg") | Some("image/jpg") => image::ImageFormat::Jpeg,
            Some("image/png") => image::ImageFormat::Png,
            Some("image/gif") => image::ImageFormat::Gif,
            Some("image/bmp") => image::ImageFormat::Bmp,
            Some("image/tiff") => image::ImageFormat::Tiff,
            _ => {
                return Err(http::ErrorCode::HttpRequestDenied);
            }
        };
        let image_bytes = request.body().blocking_read(content_length).map_err(|e| {
            http::ErrorCode::InternalError(Some(format!("Failed to read body: {:?}", e)))
        })?;
        let tm = Trustmark::new_from_bytes(MODEL, Variant::Q, Version::Bch5, true).unwrap();
        let img = image::load_from_memory(&image_bytes).map_err(|e| {
            http::ErrorCode::InternalError(Some(format!("Failed to decode image: {}", e)))
        })?;
        let output = tm.encode("0010101".to_owned(), img, 0.95).map_err(|e| {
            http::ErrorCode::InternalError(Some(format!("Failed to decode image: {}", e)))
        })?;

        let mut buf = Vec::new();
        output
            .write_to(&mut std::io::Cursor::new(&mut buf), format)
            .map_err(|e| {
                http::ErrorCode::InternalError(Some(format!("Failed to encode image: {}", e)))
            })?;
        Ok(http::Response::new(buf))
    }
}
