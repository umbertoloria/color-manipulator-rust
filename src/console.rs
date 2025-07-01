use crate::ffmpeg::ffmpeg::{compute_ffmpeg_extract_frames, FfmpegExtractFramesResult};
use std::env;

pub fn cli_init() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args); // Debug only.

    if args.len() != 1 + 4 {
        println!("Usage: EXEC [input_frames_dir] [output_frames_dir]");
        return;
    }

    if cfg!(target_os = "windows") {
        // Go ahead.
    } else {
        // TODO: Support Linux and Mac
        println!("Only Windows is supported, for now :c");
        return;
    };
    let video_dir = &args[1];
    let video_filename = &args[2];
    let time_from = &args[3];
    let time_to = &args[4];
    // let fps = &args[5];
    let fps = 30;
    // TODO: Validate args

    // FFMPEG: Extract frames
    match compute_ffmpeg_extract_frames(&video_dir, &video_filename, time_from, time_to, &fps) {
        FfmpegExtractFramesResult::AlreadyExtracted(video_file_path) => {
            println!("Video {}: frames already extracted", video_file_path);
        }
        FfmpegExtractFramesResult::CmdError(cmd_command_output) => {
            eprintln!("Failed to execute command");
            eprintln!("{:?}", &cmd_command_output);
        }
        FfmpegExtractFramesResult::OkExtract => {}
    }
}
