use base64::{engine::general_purpose, Engine};
use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::PngEncoder;
use image::{ColorType, GenericImageView, ImageEncoder, ImageReader};
use std::io::Cursor;
use svg::node::element::Image as SvgImage;
use svg::Document;
use wasm_bindgen::prelude::wasm_bindgen;

// 转换为base64
#[wasm_bindgen]
pub fn convert_to_base64(data: &[u8]) -> String {
    general_purpose::STANDARD.encode(data)
}

// 从base64转换
#[wasm_bindgen]
pub fn convert_from_base64(data: &str) -> Vec<u8> {
    general_purpose::STANDARD.decode(data).unwrap()
}

// 压缩图片
#[wasm_bindgen]
pub fn compress_image(data: &[u8], quality: u8) -> Vec<u8> {
    let reader = ImageReader::new(Cursor::new(data))
        .with_guessed_format()
        .unwrap();
    let image = reader.decode().unwrap();
    let mut output = Vec::new();

    // 使用 JPEG 编码器
    let mut binding = Cursor::new(&mut output);
    let mut encoder = JpegEncoder::new_with_quality(&mut binding, quality);
    encoder
        .encode_image(&image)
        .expect("Cannot write to JPEG format");

    output
}

// 将图像转换为 SVG
#[wasm_bindgen]
pub fn convert_to_svg(data: &[u8], quality: Option<u8>) -> String {
    // 使用 image crate 解码图像以获取宽度和高度
    let img = ImageReader::new(Cursor::new(data))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

    let (width, height) = img.dimensions();

    // 创建输出缓冲区
    let mut output = Vec::new();

    // 压缩图像（如果质量参数存在）
    if let Some(q) = quality {
        // 使用 JPEG 编码器进行压缩
        let mut encoder = JpegEncoder::new_with_quality(&mut output, q);
        encoder.encode_image(&img).expect("Cannot encode image");
    } else {
        // 如果没有质量参数，直接将图像保存到输出缓冲区（PNG 格式）
        let encoder = PngEncoder::new(&mut output);
        let rgba_image = img.to_rgba8(); // 确保图像是 RGBA8 格式
        encoder
            .write_image(
                &rgba_image,
                img.width(),
                img.height(),
                ColorType::Rgba8.into(),
            )
            .expect("Cannot encode image");
    }

    // 将图像数据转换为 Base64 编码
    let base64_data = convert_to_base64(&output);

    // 创建 SVG 图像
    let svg_image = SvgImage::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", width.to_string())
        .set("height", height.to_string())
        .set("href", format!("data:image/png;base64,{}", base64_data));

    // 创建 SVG 文档
    let document = Document::new()
        .set("width", width.to_string())
        .set("height", height.to_string())
        .add(svg_image);

    // 生成 SVG 字符串
    document.to_string()
}
