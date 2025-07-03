use crate::image_processing::image_processing::{image_processing_compute, ImageProcessingRequest};
use image::load_from_memory;
use std::collections::VecDeque;
use std::fs::{read, read_dir};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread;

pub async fn gpu_main(
    //
    input_frames_dir: String,
    output_frames_dir: String,
) {
    let frame_paths = get_png_file_paths_in_folder(&input_frames_dir).unwrap();

    let first_frame_path = &frame_paths[0];
    let frame_bytes = read(first_frame_path).unwrap();
    let frame_image = load_from_memory(&frame_bytes).unwrap().to_rgba8();
    let width = frame_image.width();
    let height = frame_image.height();

    let shared_queue: Arc<Mutex<VecDeque<ImageProcessingRequest>>> =
        Arc::new(Mutex::new(VecDeque::new()));

    let producer_queue = Arc::clone(&shared_queue);
    let producer_handle = thread::spawn(move || {
        // Frames loading
        println!("Loading {} frames", frame_paths.len());
        let mut dimensions: Option<(u32, u32)> = None;
        for frame_path_buf in &frame_paths {
            let frame_path = frame_path_buf.as_path();

            let frame_bytes = read(frame_path).unwrap();
            let frame_image = load_from_memory(&frame_bytes).unwrap().to_rgba8();
            let width = frame_image.width();
            let height = frame_image.height();

            if let Some(dimensions) = dimensions {
                if dimensions.0 != width || dimensions.1 != height {
                    eprintln!("Frame images don't have the same dimensions");
                    exit(0x0100);
                }
            } else {
                dimensions = Some((width, height));
            }

            let frame_file_name = frame_path.file_name().unwrap().to_str().unwrap();
            let output_frame_filepath = format!("{}/{}", output_frames_dir, frame_file_name);

            {
                let mut queue = producer_queue.lock().unwrap();
                queue.push_back(
                    //
                    ImageProcessingRequest {
                        image: frame_image,
                        image_output_filepath: output_frame_filepath,
                    },
                );
            }
        }
    });

    let consumer_queue = Arc::clone(&shared_queue);
    let image_processing_results =
        image_processing_compute(consumer_queue, width, height, producer_handle).await;
    println!(" -> Num of frames: {}", image_processing_results.frames);
    println!(" -> Average FPS  : {}", image_processing_results.avg_fps);
    println!(
        " -> Duration     : {}ms",
        image_processing_results.duration.as_millis()
    );
}

pub fn get_png_file_paths_in_folder(folder_path: &str) -> std::io::Result<Vec<PathBuf>> {
    let mut png_files_path_bufs = Vec::new();
    let path = Path::new(folder_path);

    // Check if the path exists and is a directory
    if !path.exists() || !path.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Folder not found or is not a directory: {}", folder_path),
        ));
    }

    for entry in read_dir(path)? {
        let entry = entry?;
        let path_buf = entry.path();

        // Check if it's a file and has a .png extension
        if path_buf.is_file() {
            if let Some(extension) = path_buf.extension() {
                if extension == "png" {
                    png_files_path_bufs.push(path_buf);
                }
            }
        }
    }
    Ok(png_files_path_bufs)
}
