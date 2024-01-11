use std::sync::{Arc, Mutex};
use std::fs;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::Instant;
use regex::Regex;

fn main() {
    let db_email = Arc::new(Mutex::new(Vec::new()));
    let db_login = Arc::new(Mutex::new(Vec::new()));
    let db_number = Arc::new(Mutex::new(Vec::new()));
    let email_regex = Regex::new(r"^\S+@\S+\.\S+$").unwrap();
    let login_regex = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_-]*$").unwrap();
    let number_regex = Regex::new(r"^\+?\d{1,4}?[-.\s]?\(?\d{1,3}?\)?[-.\s]?\d{1,4}[-.\s]?\d{1,4}[-.\s]?\d{1,9}$").unwrap();
    let mut count_lines = 0;
    let mut login_find = 0;
    let mut email_find = 0;
    let mut number_find = 0;
    let mut file = String::new();
    println!("Введите путь до файла: ");
    std::io::stdin().read_line(&mut file).unwrap();
    let start = Instant::now();
    let file_path = file.trim_matches(|c| c == '"' || c == '\n' || c == '\r');

if !Path::new(file_path).exists() {
    panic!("Файл не найден!");
}
    
    match fstream::read_lines(file_path.to_string()) {
        Some(lines) => {
            for line in lines.iter() {
                count_lines += 1;
                let line = line.trim().to_string();
                if check_print(&line) && line.matches(":").count() >= 3 {
                    let parts: Vec<&str> = line.split(":").collect();
                    let cred = parts[parts.len() - 2..].to_vec();
            
                    if cred[0].len() > 8 && cred[1].len() > 8
                        && cred[0].len() < 25 && cred[1].len() < 25
                        && !cred[0].to_lowercase().contains("http")
                        && !cred[0].to_lowercase().contains("unknown")
                        && !cred[0].to_lowercase().contains(" ")
                        && !cred[0].to_lowercase().contains("none")
                        && !cred[0].to_lowercase().contains("/")
                        && !cred[0].to_lowercase().contains("\\")
                    {
                        if email_regex.is_match(cred[0]) {
                            email_find += 1;
                            db_email.lock().unwrap().push(format!("{}:{}", cred[0], cred[1]));
                        } else if login_regex.is_match(cred[0]) {
                            login_find += 1;
                            db_login.lock().unwrap().push(format!("{}:{}", cred[0], cred[1]));
                        } else if number_regex.is_match(cred[0]) {
                            number_find += 1;
                            db_number.lock().unwrap().push(format!("{}:{}", cred[0], cred[1]));
                        }
                    }
                }
            }
        },
        None => println!("None type"),
    };
    
    let result_email = fs::File::create("result_email.txt").unwrap();
    let mut result_email = BufWriter::new(result_email);

    let result_login = fs::File::create("result_login.txt").unwrap();
    let mut result_login = BufWriter::new(result_login);

    let result_number = fs::File::create("result_number.txt").unwrap();
    let mut result_number = BufWriter::new(result_number);
    
    let db_email = Arc::clone(&db_email);
    let db_login = Arc::clone(&db_login);
    let db_number = Arc::clone(&db_number);
    
    for item in db_email.lock().unwrap().iter() {
        result_email.write_all(item.as_bytes()).unwrap();
        result_email.write_all(b"\n").unwrap();
    }
    for item in db_login.lock().unwrap().iter() {
        result_login.write_all(item.as_bytes()).unwrap();
        result_login.write_all(b"\n").unwrap();
    }
    for item in db_number.lock().unwrap().iter() {
        result_number.write_all(item.as_bytes()).unwrap();
        result_number.write_all(b"\n").unwrap();
    }
    let end = Instant::now();
    println!("Обработал файл {} с {} строк нашли {} пар\nИз них:\n    Email:pass -> {}\n    Login:pass -> {}\n    Number:pass -> {}\nЗакончил за: {:?}", file_path, count_lines, email_find + login_find + number_find, email_find, login_find, number_find, end - start);
    let _ = std::io::stdin().read_line(&mut String::new());
}

fn check_print(line: &str) -> bool {
    line.chars().all(|c| c.is_ascii() && c.is_ascii_graphic())
}
