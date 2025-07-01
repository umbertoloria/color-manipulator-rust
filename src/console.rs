use crate::ffmpeg::ffmpeg::{compute_ffmpeg_extract_frames, FfmpegExtractFramesResult};
use std::env;

const CLI_COMMAND_EXTRACT_FRAMES: &'static str = "ef";
pub fn cli_init() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args); // Debug only.

    if args.len() < 2 {
        println!("Usage: EXEC [{CLI_COMMAND_EXTRACT_FRAMES}]");
        return;
    }

    if cfg!(target_os = "windows") {
        // Go ahead.
    } else {
        // TODO: Support Linux and Mac
        println!("Only Windows is supported, for now :c");
        return;
    };
    let command = args[1].as_str();

    match command {
        CLI_COMMAND_EXTRACT_FRAMES => {
            // Extract Frames

            if args.len() != 6 {
                println!("Usage: EXEC {CLI_COMMAND_EXTRACT_FRAMES} [video_dir] [video_filename] [time_from=00:00:00] [time_to=00:00:00]");
                return;
            }

            let video_dir = &args[2];
            let video_filename = &args[3];
            let time_from = &args[4];
            let time_to = &args[5];
            // let fps = &args[5];
            let fps = 30;
            // TODO: Validate args

            // FFMPEG: Extract frames
            match compute_ffmpeg_extract_frames(
                &video_dir,
                &video_filename,
                time_from,
                time_to,
                &fps,
            ) {
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
        &_ => {
            println!("Usage: EXEC [{CLI_COMMAND_EXTRACT_FRAMES}]");
        }
    }
}
