use std::fs;
use std::path::Path;
// 34m s
fn main() {
    // ( = +1
    // ) = -1
    let file_path = Path::new("src\\input.txt");
    read_file(file_path);
}

fn read_file(file_path: &Path) {
    if file_path.is_file() {
        match fs::read(file_path) {
            Ok(chars) => {
                let mut visted_basement = false;
                let mut floor: i32 = 0;
                for (i, char) in chars.iter().enumerate() {
                    if floor == -1 && !visted_basement{
                        visted_basement = true;
                        println!("First entered the basement at: {}", i as i32);
                    }
                    match char {
                        b')' => {floor -= 1;},
                        b'(' => {floor += 1;},
                        _ => println!("invalid"),
                    }
                }
                println!("Santa will go to floor: {}", &floor);
            },
            Err(_) => println!("Failed to read file")
        }
    }
}
