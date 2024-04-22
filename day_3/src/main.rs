use std::fs;
use std::path::Path;
//1 h 32 min
fn main() {
    let file_path = Path::new("src\\input.txt");
    move_santa(file_path);
    move_santa_robo_santa(file_path);
}

fn move_santa(file_path: &Path) {
    if file_path.is_file() {
        match fs::read(file_path) {
            Ok(chars) => {
                let mut x = 0;
                let mut y = 0;
                let mut house_visted: Vec<(i32, i32)> = Vec::new();
                for char in chars {
                    match char {
                        b'>' => x += 1,
                        b'<' => x -= 1,
                        b'^' => y += 1,
                        b'v' => y -= 1,
                        _ => ()
                    }
                    let house = (x,y);

                    if !house_visted.contains(&house) {
                        house_visted.push(house)
                    }
                }
                // Add one for the starting house
                println!("Santa gave presents to {} houses",
                    house_visted.len() + 1);
            },
            Err(_) => println!("Failed to read file")
        }
    }
}

fn move_santa_robo_santa(file_path: &Path) {
    if file_path.is_file() {
        match fs::read(file_path) {
            Ok(chars) => {
                let mut x = 0;
                let mut y = 0;
                let mut r_x = 0;
                let mut r_y = 0;

                let mut santa_house_visted: Vec<(i32, i32)> = Vec::new();
                let mut robo_house_visted: Vec<(i32, i32)> = Vec::new();
                let mut house = (0,0);
                for (i, char) in chars.iter().enumerate() {
                    if i % 2 == 0 {
                        match char {
                            b'>' => x += 1,
                            b'<' => x -= 1,
                            b'^' => y += 1,
                            b'v' => y -= 1,
                            _ => ()
                        }
                        house = (x,y);
                    }
                    else {
                        match char {
                            b'>' => r_x += 1,
                            b'<' => r_x -= 1,
                            b'^' => r_y += 1,
                            b'v' => r_y -= 1,
                            _ => ()
                        }
                        house = (r_x, r_y);
                    }

                    if !santa_house_visted.contains(&house) && !robo_house_visted.contains(&house){
                        if i % 2 == 0 {
                            santa_house_visted.push(house)
                        } else {
                            robo_house_visted.push(house)
                        }
                    }
                }
                println!("Santa and Robo-Santa gave presents to {} house",
                    santa_house_visted.len()
                    + robo_house_visted.len());
            },
            Err(_) => println!("Failed to read file")
        }
    }
}
