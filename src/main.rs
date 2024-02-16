use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
mod text_utils;
mod file_utils;
mod utils;

fn main() {
    let mut log: Vec<String> = Vec::new();
    let _ = utils::clear_screen();
    let mut dir = String::new();
    if let Some((stat, dir_)) = file_utils::return_result_folder() {
        if stat {
            dir = dir_;
        }
        else {
            panic!("");
        }
    }
    if let Some((stat, path)) = file_utils::return_path() {
        match stat {
            true => {
                file_utils::start(&path, &dir);
                log.push(path.to_string())
            },
            false => {
                if let Ok(entries) = fs::read_dir(path) {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            let path = entry.path();
                            if path.is_file() {
                                file_utils::start(path.to_str().unwrap(), &dir);
                                log.push(path.to_string_lossy().to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("log.txt")
        .unwrap();
    for line in log.iter() {
        let _ = log_file.write(format!("Обработал файл: {}\n", line).as_bytes());
    }
    println!("Завершил обработку");
    let _ = std::io::stdin().read_line(&mut String::new()).unwrap();
}