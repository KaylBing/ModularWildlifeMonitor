use image::{GenericImageView, Pixel};
use std::env;

// Function to count differing pixels between two images
fn count_differing_pixels(image1_path: &str, image2_path: &str) -> u32 {
    // Load both images
    let img1 = image::open(image1_path).expect("Failed to open image 1");
    let img2 = image::open(image2_path).expect("Failed to open image 2");

    // Check if dimensions are the same
    if img1.dimensions() != img2.dimensions() {
        eprintln!("Images have different dimensions, comparison is not possible.");
        return u32::MAX; // Return an impossible high count to indicate failure
    }

    let (width, height) = img1.dimensions();
    let mut differing_pixel_count: u32 = 0;

    // Iterate over pixels and count differences
    for y in 0..height {
        for x in 0..width {
            let pixel1 = img1.get_pixel(x, y).to_rgb();
            let pixel2 = img2.get_pixel(x, y).to_rgb();

            // Check if the RGB values of the two pixels differ
            if pixel1 != pixel2 {
                differing_pixel_count += 1;
            }
        }
    }

    differing_pixel_count
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

    // Compare images and count differing pixels
    let differing_pixels = count_differing_pixels(image1_path, image2_path);

    if differing_pixels == u32::MAX {
        return; // Exit if comparison failed
    }

    // Check if the number of differing pixels is greater than 50
    if differing_pixels > 50 {
        println!("Movement detected!");
    } else {
        println!("No movement detected.");
    }
}
