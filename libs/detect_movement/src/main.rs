use opencv::{
    core::{absdiff, Mat, Scalar, Size, BORDER_DEFAULT},
    imgproc::{self, COLOR_BGR2GRAY, THRESH_BINARY},
    prelude::*,
    videoio::{VideoCapture, VideoWriter, CAP_ANY},
    highgui,
    Result,
};
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    let video_path = "/home/mikhailu/MWM/ModularWildlifeMonitor/test_footage_1280_720_30fps.mp4";
    let output_dir = "/home/mikhailu/MWM/ModularWildlifeMonitor/Flagged_Footage";

    // Create output directory if it doesn't exist
    if !Path::new(output_dir).exists() {
        fs::create_dir(output_dir).map_err(|e| opencv::Error::new(opencv::core::StsError, format!("IO Error: {}", e)))?;
    }

    // Open the video file
    let mut video = VideoCapture::from_file(video_path, CAP_ANY)?;
    if !video.is_opened()? {
        panic!("Unable to open video file!");
    }

    let mut prev_frame = Mat::default();
    let mut current_frame = Mat::default();
    let mut diff_frame = Mat::default();
    let mut frame_count = 0;

    // Variables for video writing
    let mut motion_detected = false;
    let mut video_writer: Option<VideoWriter> = None;
    let mut segment_count = 0;
    let mut no_motion_frames = 0; // Counter to track how long there is no motion

    let fps = video.get(opencv::videoio::CAP_PROP_FPS)?; // Get FPS of the video
    let width = video.get(opencv::videoio::CAP_PROP_FRAME_WIDTH)? as i32;
    let height = video.get(opencv::videoio::CAP_PROP_FRAME_HEIGHT)? as i32;
    let codec = VideoWriter::fourcc('M', 'J', 'P', 'G')?; // Codec for AVI files

    // Process frames
    loop {
        video.read(&mut current_frame)?;
        if current_frame.empty() {
            break; // End of video
        }

        frame_count += 1;

        // Convert to grayscale
        let mut gray_frame = Mat::default();
        imgproc::cvt_color(&current_frame, &mut gray_frame, COLOR_BGR2GRAY, 0)?;

        // Apply Gaussian blur
        let mut blurred_frame = Mat::default();
        imgproc::gaussian_blur(&gray_frame, &mut blurred_frame, Size::new(5, 5), 0.0, 0.0, BORDER_DEFAULT)?;
        blurred_frame.copy_to(&mut gray_frame)?;

        if !prev_frame.empty() {
            // Calculate frame difference
            absdiff(&prev_frame, &gray_frame, &mut diff_frame)?;

            // Apply threshold
            let mut threshold_frame = Mat::default();
            imgproc::threshold(&diff_frame, &mut threshold_frame, 15.0, 255.0, THRESH_BINARY)?;

            // Find contours
            let mut contours = opencv::core::Vector::<opencv::core::Vector<opencv::core::Point>>::new();
            imgproc::find_contours(
                &threshold_frame,
                &mut contours,
                imgproc::RETR_EXTERNAL,
                imgproc::CHAIN_APPROX_SIMPLE,
                opencv::core::Point::new(0, 0),
            )?;

            let mut motion_in_frame = false;

            // Check for significant motion
            for contour in contours.iter() {
                if imgproc::contour_area(&contour, false)? > 200.0 {
                    motion_in_frame = true;
                    break;
                }
            }

            // Handle motion detection
            if motion_in_frame {
                no_motion_frames = 0;

                if !motion_detected {
                    // Start a new video segment
                    motion_detected = true;
                    segment_count += 1;

                    let output_path = format!("{}/segment_{}.avi", output_dir, segment_count);
                    video_writer = Some(VideoWriter::new(&output_path, codec, fps, Size::new(width, height), true)?);
                    println!("Started recording motion segment: {}", output_path);
                }
            } else {
                no_motion_frames += 1;

                // Stop recording after a few frames of no motion
                if motion_detected && no_motion_frames > fps as i32 { // 1 second of no motion
                    println!("Stopped recording motion segment: segment_{}.avi", segment_count);
                    motion_detected = false;
                    video_writer = None;
                }
            }

            // Write frame if motion is being recorded
            if let Some(writer) = &mut video_writer {
                writer.write(&current_frame)?;
            }
        }

        // Update previous frame
        gray_frame.copy_to(&mut prev_frame)?;
    }

    Ok(())
}
