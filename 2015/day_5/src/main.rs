use std::fs;
use std::path::Path;
// 54m 08s
fn main() {
    let file_path = Path::new("src\\input.txt");
    read_file(file_path);
}

fn read_file(file_path: &Path) {
    let mut num_valid = 0u64;
    println!("{}",num_valid);
    if file_path.is_file() {
        let _: Vec<_> = fs::read_to_string(file_path)
            .unwrap()
            .lines()
            .map(str::trim).map(|x|
                if check_string(x) {
                    num_valid += 1;
                }
            ).collect();
    }
    println!("There are {} valid strings", num_valid);
}

fn check_string(line: &str) -> bool {
    if check_vowels(&line) && check_repeats(&line) && check_illegal_substrings(&line) {
        return true
    }
    else {
        return false
    }
}

// String must contain at least 3 vowels
fn check_vowels(line: &str) -> bool {
    let mut num_vowels = 0;

    for c in line.to_lowercase().chars() {
        match c {
            'a'|'e'|'i'|'o'|'u' => {
                num_vowels += 1
            },
            _ => {}
        }
        if num_vowels == 3 {
            return true
        }
    }
    return false
}

// String must contain at least one letter that repeats back to back
fn check_repeats(line: &str) -> bool {
    let mut prev_c = '\0';

    for c in line.to_lowercase().chars() {
        if prev_c == c {
            return true
        }
        else {
            prev_c = c;
        }
    }
    return false
}

// String cannot contain "ab", "cd", "pq", "xy"
fn check_illegal_substrings(line: &str) -> bool {
    let illegal = vec!["ab", "cd", "pq", "xy"];

    for pat in illegal {
        if line.to_lowercase().contains(pat) {
            return false
        }
    }
    return true
}
