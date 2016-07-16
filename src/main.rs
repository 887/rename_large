use std::fs;
use std::env;
use std::path::{PathBuf};
use std::ffi::OsString;

#[test]
fn it_works() {
    let org = PathBuf::from("\\home\\a887\\test.jpg-large");
    let expected = PathBuf::from("\\home\\a887\\test.jpg");
    let renamed = rename_path(org).unwrap();
    assert_eq!(expected, renamed);
}

fn rename_path(path_buf: PathBuf) -> Result<PathBuf, ()> {
    match path_buf.file_name() {
        Some(file_name) => {
            if file_name.to_str().unwrap().to_string().to_lowercase().ends_with("-large") {
                let mut new_buf = path_buf.clone();
                new_buf.pop();
                let mut file_name_string: String = file_name.to_str().unwrap().to_string();
                file_name_string.truncate(file_name.len() - 6);
                let file_name_os_string = OsString::from(file_name_string);
                new_buf.push(file_name_os_string);
                Ok(new_buf)
            } else {
                Err(())
            }
        },
        None => Err(())
    }
}

pub fn main() {
    let current_dir = env::current_dir().unwrap();
    for entry in fs::read_dir(current_dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_file() {
            match rename_path(path.to_path_buf()) {
                Ok(path_buf) => {
                    match fs::rename(path, path_buf.as_path()) {
                        Ok(()) => {
                            println!("{}", path_buf.file_name().unwrap().to_str().unwrap());
                        }
                        Err(_) => {},
                    }
                }
                Err(_) => {
                }
            }
        }
    }
}
