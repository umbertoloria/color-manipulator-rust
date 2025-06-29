pub fn get_path_in(name: &str) -> String {
    format!("input/{}", name)
}

pub fn get_path_out(name: &str) -> String {
    format!("out/{}", name)
}
pub fn get_path_out_gpu(name: &str) -> String {
    format!("out_gpu/{}", name)
}

/*
fn get_file_paths(dir: &str) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_file() {
            paths.push(path);
        }
    }
    paths
}
*/
