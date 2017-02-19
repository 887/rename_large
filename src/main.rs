use std::fs;
use std::env;
use std::path::{PathBuf};
use std::ffi::OsString;

#[test]
fn it_works() {
    let org = PathBuf::from("\\home\\a887\\test.jpg-large");
    let expected = PathBuf::from("\\home\\a887\\test.jpg");
    let renamed = rename_path(org);
    assert_eq!(Some(expected), renamed);
}

fn rename_path(path_buf: PathBuf) -> Option<PathBuf> {
    if let Some(file_name) = path_buf.file_name() {
        let mut file_name_string = file_name.to_str().unwrap().to_lowercase();
        if file_name_string.ends_with("-small") || file_name_string.ends_with("-large") {
            file_name_string.truncate(file_name.len() - 6);
            let mut new_buf = path_buf.clone();
            new_buf.pop();
            let file_name_os_string = OsString::from(file_name_string);
            new_buf.push(file_name_os_string);
            return Some(new_buf)
        }
    }
    None
}

pub fn main() {
    let current_dir = env::current_dir().unwrap();
    for entry in fs::read_dir(current_dir).unwrap() {
        let path_buf = entry.unwrap().path();
        if path_buf.is_file() {
            if let Some(new_buf) = rename_path(path_buf.to_path_buf()) {
                if new_buf.as_path().exists() {
                    fs::remove_file(path_buf.as_path()).unwrap_or_else(|_|{});
                } else {
                    fs::rename(path_buf.as_path(), new_buf.as_path()).unwrap_or_else(|_|{});
                }
                println!("{}", path_buf.file_name().unwrap().to_str().unwrap().to_string());
            }
        }
    }
}
