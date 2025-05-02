use image::{DynamicImage, ImageBuffer, RgbaImage, ImageFormat};
use std::path::Path;

pub fn generate_icons() {
    let sizes = [32, 128, 256];
    let base_image = image::open("assets/genshin.jpg").expect("Failed to open base image");
    
    // 确保输出目录存在
    std::fs::create_dir_all("icons").expect("Failed to create icons directory");
    
    for size in sizes.iter() {
        let resized = base_image.resize(*size, *size, image::imageops::FilterType::Lanczos3);
        // 转换为 RGBA 格式并保存为 PNG
        let rgba = resized.into_rgba8();
        rgba.save_with_format(
            format!("icons/{}x{}.png", size, size),
            ImageFormat::Png
        ).expect("Failed to save icon");
        
        // 生成 2x 版本
        if *size == 128 {
            let resized_2x = base_image.resize(*size * 2, *size * 2, image::imageops::FilterType::Lanczos3);
            let rgba_2x = resized_2x.into_rgba8();
            rgba_2x.save_with_format(
                format!("icons/{}x{}@2x.png", size, size),
                ImageFormat::Png
            ).expect("Failed to save 2x icon");
        }
    }
    
    // 生成 .ico 文件 (Windows)
    let mut ico = ico::IconDir::new(ico::ResourceType::Icon);
    let sizes = [16, 32, 48, 256];
    for size in sizes.iter() {
        let resized = base_image.resize(*size, *size, image::imageops::FilterType::Lanczos3);
        // 确保转换为 RGBA 格式
        let rgba = resized.into_rgba8();
        let raw_data = rgba.into_raw();
        let icon = ico::IconImage::from_rgba_data(*size, *size, raw_data);
        ico.add_entry(ico::IconDirEntry::encode(&icon).expect("Failed to encode icon"));
    }
    let mut ico_file = std::fs::File::create("icons/icon.ico").expect("Failed to create .ico file");
    ico.write(&mut ico_file).expect("Failed to write .ico file");
} 