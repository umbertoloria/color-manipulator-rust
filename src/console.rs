use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

const ALREADY_COMPUTED_FILE_NAME_SUFFIX: &'static str = "_already_computed.txt";
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

    // VIDEO FILE PATH
    let video_file_path = &format!("{}/{}", video_dir, video_filename);

    // ALREADY COMPUTED CHECK
    let already_computed_file_path = &format!(
        "{}/{}{}",
        video_dir, video_filename, ALREADY_COMPUTED_FILE_NAME_SUFFIX
    );
    let path = Path::new(already_computed_file_path);
    if path.exists() {
        println!("Video {}: frames already extracted", video_file_path);
        return;
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
        // TODO: Errors are not signaled by ffmpeg cli
        eprintln!("Failed to execute command");
        eprintln!("{:?}", &cmd_command_output);
        return;
    }

    // ALREADY COMPUTED WRITE
    let mut file = File::create(already_computed_file_path).unwrap();
    file.write_all(b".").unwrap();
    file.flush().unwrap();
}
