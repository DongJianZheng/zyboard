use md5::{Digest, Md5};
use image::{ImageBuffer, RgbaImage};
use image::codecs::jpeg::JpegEncoder;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

pub fn md5_hash(content: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn md5_hash_bytes(bytes: &[u8]) -> String {
    let mut hasher = Md5::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

pub fn rgba8_to_base64(image: &RgbaImage) -> String {
    let width = image.width();
    let height = image.height();

    let rgba: Vec<u8> = image
        .pixels()
        .flat_map(|pixel| {
            [pixel[0], pixel[1], pixel[2], pixel[3]]
        })
        .collect();

    let header = format!(
        "{{\"width\":{},\"height\":{},\"base64\":\"{}\"}}",
        width,
        height,
        BASE64.encode(&rgba)
    );

    header
}

pub fn rgba8_to_jpeg_base64(image: &RgbaImage, quality: u8) -> String {
    let width = image.width();
    let height = image.height();

    // 转换为 RGB
    let rgb_image: Vec<u8> = image
        .pixels()
        .flat_map(|pixel| [pixel[0], pixel[1], pixel[2]])
        .collect();

    let img: ImageBuffer<image::Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_raw(width, height, rgb_image).unwrap();

    let mut jpeg_data = Vec::new();
    let mut encoder = JpegEncoder::new_with_quality(&mut jpeg_data, quality);
    encoder
        .encode(
            img.as_raw(),
            width,
            height,
            image::ColorType::Rgb8,
        )
        .unwrap();

    let header = format!(
        "{{\"width\":{},\"height\":{},\"base64\":\"{}\"}}",
        width,
        height,
        BASE64.encode(&jpeg_data)
    );

    header
}

pub fn base64_to_rgba8(data: &str) -> Result<RgbaImage, String> {
    // 解析 JSON 格式
    let start_idx = data.find('{').ok_or("无效的数据格式")?;
    let end_idx = data.rfind('}').ok_or("无效的数据格式")?;
    let json_str = &data[start_idx..=end_idx];

    #[derive(serde::Deserialize)]
    struct ImageData {
        width: u32,
        height: u32,
        base64: String,
    }

    let image_data: ImageData =
        serde_json::from_str(json_str).map_err(|e| format!("解析 JSON 失败: {}", e))?;

    let bytes = BASE64.decode(&image_data.base64)
        .map_err(|e| format!("Base64 解码失败: {}", e))?;

    // 转换为 RGBA
    let mut rgba_bytes = Vec::with_capacity(bytes.len() / 3 * 4);
    for chunk in bytes.chunks(3) {
        rgba_bytes.push(chunk[0]);
        rgba_bytes.push(chunk[1]);
        rgba_bytes.push(chunk[2]);
        rgba_bytes.push(255);
    }

    RgbaImage::from_raw(image_data.width, image_data.height, rgba_bytes)
        .ok_or("无效的图像数据".to_string())
}
