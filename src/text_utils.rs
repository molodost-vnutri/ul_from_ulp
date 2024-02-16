use regex::Regex;

pub fn check_cred(data: &str, password: &str, email: &Regex, login: &Regex, number: &Regex, bad_word: Vec<&str>) -> i8 {
    if password.chars().all(|c| c.is_ascii() && c.is_ascii_graphic()) {
        for bad in bad_word.iter() {
            if bad.contains(data) { return 3; }
        }
        if email.is_match(data) { return 0; }
        if login.is_match(data) { return 1; }
        if number.is_match(data) { return 2; }
    }
    3
}