use std::env::current_dir;

pub fn make_safe_filepath(filepath_str: &str) -> String {
    let mut filepath = current_dir().unwrap();
    filepath.push(filepath_str);
    filepath.into_os_string().into_string().unwrap()
}
