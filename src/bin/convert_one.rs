use eyre::ContextCompat;
use image::ImageFormat;
use image_converter_rs::convert_image;
use std::env;

fn main() -> eyre::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input file path> <output file path>", args[0]);
        eprintln!(
            "e.g.: {} <pictures/test.jpg> <pictures/converted/test.ico>",
            args[0]
        );
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    let output_format =
        ImageFormat::from_extension(output_path).wrap_err("Error parsing output extension")?;

    match convert_image(input_path, output_path, output_format) {
        Ok(_) => println!("Successfully converted {input_path} to {output_format:?}",),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
