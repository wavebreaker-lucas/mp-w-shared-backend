use screenshots::Screen;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use std::io::Cursor;
use image::codecs::jpeg::JpegEncoder;

pub fn capture_screenshot(x: i32, y: i32) -> Option<String> {
    let screens = Screen::all().ok()?;
    
    let screen = screens.iter().find(|screen| {
        let display_info = screen.display_info;
        x >= display_info.x as i32
            && x < display_info.x as i32 + display_info.width as i32
            && y >= display_info.y as i32
            && y < display_info.y as i32 + display_info.height as i32
    })?;

    let image = screen.capture().ok()?;
    
    let mut buffer = Cursor::new(Vec::new());
    let mut encoder = JpegEncoder::new_with_quality(&mut buffer, 85);
    encoder.encode(
        image.as_raw(),
        image.width(),
        image.height(),
        image::ColorType::Rgba8
    ).ok()?;
    
    let base64_string = BASE64.encode(buffer.get_ref());
    Some(base64_string)
}