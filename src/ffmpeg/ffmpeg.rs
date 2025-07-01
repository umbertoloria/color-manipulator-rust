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
    fps: &i32,
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
    // ORIGINAL COMMAND: "ffmpeg -ss %1 -to %2 -i %3 -vf fps=%4 %5\..._frame_%%04d.png"
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
