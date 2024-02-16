use colored::Colorize;
use std::{fs::{self, OpenOptions}, io::{BufRead, BufReader, Write}, path::Path};
use std::fs::File;
use regex::Regex;
use chrono;

use crate::text_utils;
use crate::utils;

pub fn return_result_folder() -> Option<(bool, String)> {
    let time_dir = chrono::offset::Local::now();
    let normalize_dir = format!("Result_{}",time_dir.format("%Y_%m_%d_%H"));
    if Path::new(normalize_dir.as_str()).exists() {
        return Some((true, normalize_dir.to_string()));
    }
    match fs::create_dir(normalize_dir.clone()) {
        Ok(_) => return Some((true, normalize_dir.to_string())),
        Err(_) => println!("Не удалось создать папку {}", normalize_dir)
    }
    return Some((false, normalize_dir))
}

pub fn start(file: &str, result_dir: &str) {

    let file_open = File::open(file).expect(format!("Не удалось открыть файл {}", file).as_str());
    let reader = BufReader::new(file_open);

    let mut email_db: Vec<String> = Vec::new();
    let mut login_db: Vec<String> = Vec::new();
    let mut number_db: Vec<String> = Vec::new();

    let mut email_count: u128 = 0;
    let mut login_count: u128 = 0;
    let mut number_count: u128 = 0;

    let email_regex = Regex::new(r"^\S+@\S+\.\S+$").unwrap();
    let login_regex = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_-]*$").unwrap();
    let number_regex = Regex::new(r"^\+?\d{1,4}?[-.\s]?\(?\d{1,3}?\)?[-.\s]?\d{1,4}[-.\s]?\d{1,4}[-.\s]?\d{1,9}$").unwrap();
    let bad_word = vec!["unknown", "null", "none"];
    let charses = vec![";", " |", "| ", "|", " ", ","];

    for line in BufReader::lines(reader) {
        match line {
            Ok(line) => {
                if check_size(&email_db, &login_db, &number_db) {
                    if let Some((email_counts, login_counts, number_counts)) = write(&email_db, &login_db, &number_db, &result_dir) {
                        email_count += email_counts;
                        login_count += login_counts;
                        number_count += number_counts;
                        email_db.clear();
                        login_db.clear();
                        number_db.clear();
                        let _ = utils::clear_screen();
                        println!("С файла: {}\n    [email:password]=>  {}\n    [login:password]=> {}\n    [number:password]=> {}", file, email_count.to_string().green(), login_count.to_string().green(), number_count.to_string().green());
                    }
                }
                let mut line = line;
                for char in charses.iter() {
                    line = line.replace(char, ":");
                }
                let parts: Vec<&str> = line.split(":").collect();
                let cred = parts[parts.len() - 2..].to_vec();
                match text_utils::check_cred(cred[0], cred[1], &email_regex, &login_regex, &number_regex, bad_word.clone()) {
                    0 => email_db.push(format!("{}:{}", cred[0], cred[1])),
                    1 => login_db.push(format!("{}:{}", cred[0], cred[1])),
                    2 => number_db.push(format!("{}:{}", cred[0], cred[1])),
                    _ => {}
                }

            }
            Err(line) => {
                println!("Возникла проблема при чтении строки {}, она будет записана по пути {}/error_string.txt", line, result_dir);
                let mut error_string = OpenOptions::new()
                    .append(true)
                    .write(true)
                    .create(true)
                    .open(format!("{}/error_string.txt", result_dir).as_str())
                    .expect(format!("Не удалось создать файл error_string.txt в папке {}", result_dir).as_str());
                let _ = error_string.write(format!("{}\n", line).as_bytes());
            }
        }
    }
    if let Some((email_counts, login_counts, number_counts)) = write(&email_db, &login_db, &number_db, result_dir) {
        email_count += email_counts;
        login_count += login_counts;
        number_count += number_counts;
        email_db.clear();
        login_db.clear();
        number_db.clear();
        let _ = utils::clear_screen();
        println!("С файла: {}\n    [email:password]=>  {}\n    [login:password]=> {}\n    [number:password]=> {}", file, email_count.to_string().green(), login_count.to_string().green(), number_count.to_string().green());
    }
}

fn check_size(email_db: &Vec<String>, login_db: &Vec<String>, number_db: &Vec<String>) -> bool {
    if email_db.len() + login_db.len() + number_db.len() >= 100000 {
        return true;
    }
    false
}

fn write(email_db: &Vec<String>, login_db: &Vec<String>, number_db: &Vec<String>, result_dir: &str) -> Option<(u128, u128, u128)> {
    let email_counts: u128 = email_db.len().try_into().unwrap();
    let login_counts: u128 = login_db.len().try_into().unwrap();
    let number_counts: u128 = number_db.len().try_into().unwrap();

    let mut email_result = OpenOptions::new()
        .append(true)
        .write(true)
        .create(true)
        .open(format!("{}/email_result.txt", result_dir).as_str())
        .expect(format!("Не удалось создать файл {}/email_result.txt", result_dir).as_str());
    for line in email_db.iter() {
        let _ = email_result.write(format!("{}\n", line.to_string()).as_bytes()).expect(format!("Не удалось записать пару: {}", line).as_str());
    }
    let mut login_result = OpenOptions::new()
        .append(true)
        .write(true)
        .create(true)
        .open(format!("{}/login_result.txt", result_dir))
        .expect(format!("Не удалось создать файл {}/login_result.txt", result_dir).as_str());
    for line in login_db.iter() {
        let _ = login_result.write(format!("{}\n", line).as_bytes()).expect(format!("Не удалсь записать пару: {}", line).as_str());
    }
    let mut number_result = OpenOptions::new()
        .append(true)
        .write(true)
        .create(true)
        .open(format!("{}/number_result.txt", result_dir))
        .expect(format!("Не удалось создать файл {}/number_result.txt", result_dir).as_str());
    for line in number_db.iter() {
        let _ = number_result.write(format!("{}\n", line).as_bytes()).expect(format!("Не удалось создать файл {}/number_result.txt", result_dir).as_str());
    }
    return Some((email_counts, login_counts, number_counts))
}

pub fn return_path() -> Option<(bool, String)> {
    loop {
        let mut path = String::new();
        let _ = utils::clear_screen();
        print!("[Path]=> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut path).unwrap();

        path = path.trim().replace("& '", "").replace("'", "");
        path.retain(|c| c != '"');

        let link_path = Path::new(&path);
        
        if link_path.exists() {
            if link_path.is_dir() {
                return Some((false, path));
            } else if link_path.is_file() {
                return Some((true, path));
            } 
        } else { 
            println!("Путь {} не найден", path.trim());
            let _ = std::io::stdin().read_line(&mut String::new());
        }
    }
}