use std::{fs, path::PathBuf};

pub fn find_path(paths: Vec<PathBuf>, cmd: String) -> Option<PathBuf> {
    for path in paths {
        let mut files = fs::read_dir(path.clone()).unwrap();
        if files.any(|file| file.unwrap().file_name() == cmd.as_str()) {
            return Some([path, PathBuf::from(cmd.as_str())].iter().collect());
        }
    }

    None
}
