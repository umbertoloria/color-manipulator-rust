pub fn get_path_out(name: &str) -> String {
    format!("out/{}", name)
}

pub fn get_path_oracle(name: &str) -> String {
    format!("oracle/{}", name)
}

pub fn get_path_oracle_diff(name: &str) -> String {
    format!("out/oracle_diff/{}", name)
}
