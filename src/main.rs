use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_image>", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let input_path = Path::new(input_path);
    
    // Load the image
    let img = match image::open(input_path) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("Failed to open image: {}", e);
            std::process::exit(1);
        }
    };

    // Convert to RGB8
    let rgb_img = img.to_rgb8();
    let (width, height) = rgb_img.dimensions();
    
    // Prepare AVIF encoder configuration
    let encoder = ravif::Encoder::new()
        .with_quality(80.0)
        .with_alpha_quality(80.0)
        .with_speed(6);

    let rgb_pixels: Vec<rgb::RGB8> = rgb_img
        .pixels()
        .map(|p| rgb::RGB8::new(p[0], p[1], p[2]))
        .collect();

    // Create Img struct for ravif
    let img = ravif::Img::new(&rgb_pixels[..], width as usize, height as usize);
    
    let encoded = match encoder.encode_rgb(img) {
        Ok(encoded) => encoded,
        Err(e) => {
            eprintln!("Failed to encode AVIF: {}", e);
            std::process::exit(1);
        }
    };

    // Generate output filename
    let output_filename = input_path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string(); // Convert Cow<str> to String
    let output_filename = output_filename + ".avif";

    // Save the file
    if let Err(e) = std::fs::write(&output_filename, encoded.avif_file) {
        eprintln!("Failed to write output file: {}", e);
        std::process::exit(1);
    }

    println!("Successfully converted to: {}", output_filename);
}
