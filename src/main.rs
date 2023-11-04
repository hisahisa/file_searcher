use std::os::unix::fs::MetadataExt;
use std::env;
use std::fs::{ReadDir, read_dir};

fn main() {

    let vec_args: Vec<String> = env::args().collect();
    let args: &str = &vec_args[1..][0];

    let paths:ReadDir = read_dir(args).unwrap();
    let mut vec_path: Vec<(String, String, u64)> = Vec::new();
    let mut vec_file: Vec<(String, String, u64)> = Vec::new();

    get_path_info(paths, &mut vec_path);
    for (_, path, _) in &vec_path {
        match read_dir(&path[..]) {
            Ok(path) => {
                let file_list = get_file_info(path,true);
                vec_file.extend(file_list);
            }
            Err(e) => {println!("error = {}, reason = {}", path, e)}
        }
    }

    vec_file.sort_by(|a, b| b.2.cmp(&a.2));
    for (_, path, entry) in vec_file.iter().take(100){
        println!("{}  {} byte", path, entry);
    }
}

fn get_path_info(paths: ReadDir, vec_path: &mut Vec<(String, String, u64)>) {
    let vec_ = get_file_info(paths, false);
    if vec_.len() > 0 {
        for (_, path, _) in &vec_ {
            match read_dir(&path[..]) {
                Ok(path) => {get_path_info(path, vec_path);}
                Err(e) => {println!("error = {}, reason = {}", path, e)}
            }
        }
        vec_path.extend(vec_);
    }
}

fn get_file_info(paths: ReadDir, is_file: bool) -> Vec<(String, String, u64)> {
    let vec_=  paths
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|entry| (entry.file_name(), entry.path(), entry ))
        .filter(|entry| entry.2.metadata().ok().is_some() )
        .filter(|entry| {
            if is_file {
                !entry.2.metadata().unwrap().is_dir()
            } else {
                entry.2.metadata().unwrap().is_dir()
            }
        })
        .map(|entry| {
            (
                entry.0.into_string().unwrap(),
                entry.1.into_os_string().into_string().unwrap(),
                entry.2.metadata().unwrap().size()
            )
        })
        .collect::<Vec<_>>();
    vec_
}
