use opencv::{
    core::{absdiff, add_weighted, Mat, Scalar, Size, BORDER_DEFAULT},
    imgproc::{self, COLOR_BGR2GRAY, THRESH_BINARY},
    prelude::*,
    videoio::{VideoCapture, CAP_ANY},
    highgui,
    Result,
};

fn main() -> Result<()> {
    let video_path = "/home/mikhailu/MWM/ModularWildlifeMonitor/test_footage_1280_720_30fps.mp4";

    // Open the video file as mutable if possible //
    let mut video = VideoCapture::from_file(video_path, CAP_ANY)?;
    if !video.is_opened()? {
        panic!("Unable to open video file!");
    }

    let mut prev_frame = Mat::default();
    let mut current_frame = Mat::default();
    let mut diff_frame = Mat::default();
    let mut frame_count = 0;

    // Infinite loop to read and process frames as long as they exist //
    loop {
        // Capture the next frame //
        video.read(&mut current_frame)?;

        // Break the loop if the video ends //
        if current_frame.empty() {
            break;
        }

        // Skip every other frame (processes 15 fps if video is 30 fps) //
        if frame_count % 2 != 0 {
            frame_count += 1;
            continue;
        }
        frame_count += 1;

        // Convert to grayscale //
        let mut gray_frame = Mat::default();
        imgproc::cvt_color(&current_frame, &mut gray_frame, COLOR_BGR2GRAY, 0)?;

        // Apply Gaussian blur with a smaller kernel //
        let mut blurred_frame = Mat::default();
        imgproc::gaussian_blur(&gray_frame, &mut blurred_frame, Size::new(5, 5), 0.0, 0.0, BORDER_DEFAULT)?;

        // Replace gray_frame with blurred_frame //
        blurred_frame.copy_to(&mut gray_frame)?;

        if !prev_frame.empty() {
            // Calculate the difference between the current and previous frame //
            absdiff(&prev_frame, &gray_frame, &mut diff_frame)?;

            // Apply threshold with a lower sensitivity value //
            let mut threshold_frame = Mat::default();
            imgproc::threshold(&diff_frame, &mut threshold_frame, 15.0, 255.0, THRESH_BINARY)?;

            // Debugging: Show intermediate frames //
            highgui::imshow("Difference Frame", &diff_frame)?;
            highgui::imshow("Threshold Frame", &threshold_frame)?;

            // Find contours and draw regions //
            let mut contours = opencv::core::Vector::<opencv::core::Vector<opencv::core::Point>>::new();
            imgproc::find_contours(
                &threshold_frame,
                &mut contours,
                imgproc::RETR_EXTERNAL,
                imgproc::CHAIN_APPROX_SIMPLE,
                opencv::core::Point::new(0, 0),
            )?;

            // Draw contours to highlight detected motion //
            for contour in contours.iter() {
                if imgproc::contour_area(&contour, false)? > 200.0 {
                    imgproc::draw_contours(
                        &mut current_frame,
                        &contours,
                        -1,
                        Scalar::new(0.0, 255.0, 0.0, 0.0),
                        2,
                        imgproc::LINE_8,
                        &opencv::core::no_array(),
                        i32::MAX,
                        opencv::core::Point::new(0, 0),
                    )?;
                }
            }

            // Show the result in a window //
            highgui::imshow("Motion Detection in Video", &current_frame)?;
        }

        // Copy the current frame to previous frame for the next iteration //
        gray_frame.copy_to(&mut prev_frame)?;

        // Break the loop if 'q' key is pressed //
        let key = highgui::wait_key(30)?; // Wait 30ms between frames
        if key == 113 { // ASCII value of 'q'
            break;
        }
    }

    Ok(())
}
