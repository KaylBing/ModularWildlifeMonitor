use image::{GenericImageView, Pixel};
use std::env;

// Function to calculate the sum of RGB values of all pixels in an image
fn image_sum(img_path: &str) -> u32 {
    // Load image
    let img = image::open(img_path).expect("Failed to open image");

    // Get image dimensions
    let (width, height) = img.dimensions();
    // println!("Image dimensions: {}x{}", width, height);

    let mut full_sum: u32 = 0;

    // Iterate over pixels
    for y in 0..height {
        for x in 0..width {
            // Get the pixel at (x, y)
            let pixel = img.get_pixel(x, y);
            let rgb = pixel.to_rgb();
            let (r, g, b) = (rgb[0] as u32, rgb[1] as u32, rgb[2] as u32); // Load u8s as u32s
            full_sum += r + g + b;
        }
    }

    full_sum
}

// Function to compare two images based on their pixel sums
fn compare_images(image1_path: &str, image2_path: &str) -> bool {
    // Get the sum of RGB values for both images
    let sum1 = image_sum(image1_path);
    let sum2 = image_sum(image2_path);

    // Calculate the difference
    let difference = (sum1 as i32 - sum2 as i32).abs();

    // Check if the difference is greater than 5000
    difference > 5000
}

fn main() {
    // Summon command line arguments
    let args: Vec<String> = env::args().collect();

    // Check for paths of two images
    if args.len() < 3 {
        eprintln!("Usage: {} <image1_path> <image2_path>", args[0]);
        return;
    }

    let image1_path = &args[1];
    let image2_path = &args[2];

    // Compare images and print the result
    if compare_images(image1_path, image2_path) {
        println!("Movement detected!"); // Will be replaced with true or false later
    } else {
        println!("No movement detected.");
    }
}
