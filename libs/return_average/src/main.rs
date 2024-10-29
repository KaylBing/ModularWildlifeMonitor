use image::{GenericImageView, Pixel};
use std::env;

// Returns the sum numbers (RGB) of all pixels in an image //
fn image_sum(){
    // Summon command line arguments //
    let args: Vec<String> = env::args().collect();

    // Check for path //
    if args.len() < 2 {
        eprintln!("Usage: {} <image_path>", args[0]);
        return;
    }

    let img_path = &args[1];
    // Load image //
    let img = image::open(img_path).expect("Failed to open image");

    // Get edgd dimensions //
    let (width, height) = img.dimensions();
    println!("Image dimensions: {}x{}", width, height); // Temp

    let mut full_sum: u32 = 0;

    // Iterate over pixels //
    for y in 0..height {
        for x in 0..width {
            // Get the pixel at (x, y)
            let pixel = img.get_pixel(x, y);
            let rgb = pixel.to_rgb();
            let (r, g, b) = (rgb[0] as u32, rgb[1] as u32, rgb[2] as u32); // Load u8s as u32s //
            full_sum += r + g + b;
        }
    }
    println!("{}", full_sum);
}

fn main(){
    image_sum()
}
