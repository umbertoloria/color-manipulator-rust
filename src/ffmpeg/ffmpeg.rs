use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Output};

pub enum FfmpegExtractFramesResult {
    AlreadyExtracted(String),
    CmdError(Output),
    OkExtract,
}
const ALREADY_COMPUTED_FILE_NAME_SUFFIX: &'static str = "_already_computed.txt";
pub fn compute_ffmpeg_extract_frames(
    video_dir: &str,
    video_filename: &str,
    time_from: &str,
    time_to: &str,
    fps: usize,
) -> FfmpegExtractFramesResult {
    let video_file_path = &format!("{}/{}", video_dir, video_filename);

    // ALREADY COMPUTED FILE: CHECK
    let already_computed_file_path = &format!(
        "{}/{}{}",
        video_dir, video_filename, ALREADY_COMPUTED_FILE_NAME_SUFFIX
    );
    let path = Path::new(already_computed_file_path);
    if path.exists() {
        return FfmpegExtractFramesResult::AlreadyExtracted(video_file_path.into());
    }

    // FFMPEG
    println!("Video {}: extracting frames...", video_file_path);
    // ORIGINAL COMMAND: "ffmpeg -ss %1 -to %2 -i %3 -vf fps=%4 %5\..._frame_%04d.png"
    let mut cmd_command = Command::new("cmd");
    cmd_command.args([
        //
        "/C",
        "ffmpeg",
        "-ss",
        time_from,
        "-to",
        time_to,
        "-i",
        video_file_path,
        "-vf",
        &format!("fps={}", fps),
        &format!("{}/{}_frame_%04d.png", video_dir, video_filename),
    ]);
    // println!("{:?}", &cmd_command); // Debug only.
    let cmd_command_output = cmd_command.output().expect("failed to execute process");
    if !cmd_command_output.status.success() {
        return FfmpegExtractFramesResult::CmdError(cmd_command_output);
    }

    // ALREADY COMPUTED FILE: WRITE
    let mut file = File::create(already_computed_file_path).unwrap();
    file.write_all(b".").unwrap();
    file.flush().unwrap();
    FfmpegExtractFramesResult::OkExtract
}

pub enum FfmpegComposeVideoResult {
    CmdError(Output),
    OkComposed,
}
pub fn compute_ffmpeg_compose_video(
    frames_dir: &str,
    frames_fps: usize,
    src_video_filename: &str,
    out_video_filename: &str,
) -> FfmpegComposeVideoResult {
    let video_file_path = &format!("{}/{}", frames_dir, out_video_filename);
    let frame_file_path_expr = &format!("{}/{}_frame_%04d.png", frames_dir, src_video_filename);

    // FFMPEG
    println!("Video {}: composing video...", video_file_path);
    // ORIGINAL COMMAND: "ffmpeg -framerate 30 -i %5\..._frame_%04d.png -c:v libx264 -pix_fmt yuv420p output.mp4"
    let mut cmd_command = Command::new("cmd");
    cmd_command.args([
        //
        "/C",
        "ffmpeg",
        "-y", // TODO: Overwriting composed videos by default (ask before)
        "-framerate",
        &format!("{}", frames_fps),
        "-i",
        frame_file_path_expr,
        "-c:v",
        "libx264",
        "-pix_fmt",
        "yuv420p",
        video_file_path,
    ]);
    // println!("{:?}", &cmd_command); // Debug only.
    let cmd_command_output = cmd_command.output().expect("failed to execute process");
    if !cmd_command_output.status.success() {
        return FfmpegComposeVideoResult::CmdError(cmd_command_output);
    }

    FfmpegComposeVideoResult::OkComposed
}
