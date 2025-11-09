use eyre::ContextCompat;
use image::ImageFormat;
use image_converter_rs::{collect_image_files, convert_image};
use std::{env, path::PathBuf, str::FromStr};

fn main() -> eyre::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!(
            "Usage: {} <input folder> <output folder> <output type>",
            args[0]
        );
        eprintln!("e.g.: {} pictures/ pictures/converted/ ico", args[0]);
        std::process::exit(1);
    }

    let input_folder = PathBuf::from_str(&args[1])?;
    let output_folder = PathBuf::from_str(&args[2])?;
    let output_extension = &args[3];

    let output_format =
        ImageFormat::from_extension(output_extension).wrap_err("Error parsing output extension")?;

    let input_images = collect_image_files(input_folder);

    for image_file in &input_images {
        match convert_image(
            image_file,
            output_folder.join(format!(
                "{}.{}",
                image_file
                    .file_stem()
                    .wrap_err("Error getting input file name without extension")?
                    .display(),
                output_format.extensions_str()[0]
            )),
            output_format,
        ) {
            Ok(_) => println!(
                "Successfully converted {image_file:?} to {output_format:?} in {output_folder:?}",
            ),
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }

    Ok(())
}
