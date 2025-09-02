use image::ImageFormat;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        eprintln!("Usage: {} <input.webp> <output.jpg>", args[0]);
        std::process::exit(1);
    }
    
    let input_path = &args[1];
    let output_path = &args[2];
    
    match convert_webp_to_jpg(input_path, output_path) {
        Ok(_) => println!("Successfully converted {} to {}", input_path, output_path),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn convert_webp_to_jpg(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open(input_path)?;
    img.save_with_format(output_path, ImageFormat::Jpeg)?;
    Ok(())
}
