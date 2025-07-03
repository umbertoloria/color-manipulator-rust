use crate::ffmpeg::ffmpeg::{
    compute_ffmpeg_compose_video, compute_ffmpeg_extract_frames, FfmpegComposeVideoResult,
    FfmpegExtractFramesResult,
};
use crate::gpu_main::gpu_main;
use std::env;

const CLI_COMMAND_EXTRACT_FRAMES: &'static str = "ef";
const CLI_COMMAND_COMPUTE_FRAMES: &'static str = "cf";
const CLI_COMMAND_COMPOSE_VIDEO: &'static str = "cv";

pub const FPS: usize = 30;

pub fn cli_init() {
    // Examples:
    //  => cargo run --release ef ./video_project video.mkv 00:05:55 00:06:02
    //  => cargo run --release cf ./video_project ./video_project/result
    //  => cargo run --release cv ./video_project/result video.mkv output.mp4

    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args); // Debug only.

    if args.len() < 2 {
        println!("Usage: EXEC [{CLI_COMMAND_EXTRACT_FRAMES}|{CLI_COMMAND_COMPUTE_FRAMES}|{CLI_COMMAND_COMPOSE_VIDEO}]");
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

            // CLI
            if args.len() != 6 {
                println!("Usage: EXEC {CLI_COMMAND_EXTRACT_FRAMES} [video_dir] [video_filename] [time_from=00:00:00] [time_to=00:00:00]");
                return;
            }
            let video_dir = &args[2];
            let video_filename = &args[3];
            let time_from = &args[4];
            let time_to = &args[5];
            // let fps = &args[5];
            // TODO: Validate args

            // FFMPEG: Extract frames
            match compute_ffmpeg_extract_frames(
                &video_dir,
                &video_filename,
                time_from,
                time_to,
                FPS,
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

        CLI_COMMAND_COMPUTE_FRAMES => {
            // Compute Frames

            // CLI
            if args.len() != 4 {
                println!("Usage: EXEC {CLI_COMMAND_COMPUTE_FRAMES} [input_frames_dir] [output_frames_dir]");
                return;
            }
            let input_frames_dir = args[2].as_str();
            let output_frames_dir = args[3].as_str();

            pollster::block_on(
                //
                gpu_main(
                    //
                    input_frames_dir.into(),
                    output_frames_dir.into(),
                ),
            );
        }

        CLI_COMMAND_COMPOSE_VIDEO => {
            // Compose Video

            // CLI
            if args.len() != 5 {
                println!("Usage: EXEC {CLI_COMMAND_EXTRACT_FRAMES} [frames_dir] [src_video_filename] [out_video_filename]");
                return;
            }
            let frames_dir = &args[2];
            let src_video_filename = &args[3];
            let out_video_filename = &args[4];
            // TODO: Validate args

            // FFMPEG: Extract frames
            match compute_ffmpeg_compose_video(
                &frames_dir,
                FPS,
                &src_video_filename,
                &out_video_filename,
            ) {
                FfmpegComposeVideoResult::CmdError(cmd_command_output) => {
                    eprintln!("Failed to execute command");
                    eprintln!("{:?}", &cmd_command_output);
                }
                FfmpegComposeVideoResult::OkComposed => {}
            }
        }

        &_ => {
            println!("Usage: EXEC [{CLI_COMMAND_EXTRACT_FRAMES}|{CLI_COMMAND_COMPUTE_FRAMES}|{CLI_COMMAND_COMPOSE_VIDEO}]");
        }
    }
}
