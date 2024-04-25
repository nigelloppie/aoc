use std::fs::read_to_string;
use std::path::Path;
use std::vec;
// 36m 02s
fn main() {
    let file_path = Path::new("src\\input.txt");
    read_file(file_path);
}

fn read_file(file_path: &Path) {
    if file_path.is_file() {
        let mut total_wrapping = 0;
        let mut total_ribbon = 0;
        for line in read_to_string(file_path).unwrap().lines() {
            let mut deminsions = line.split("x");
            let length: u32 = Result::expect(deminsions.next().unwrap().parse(),"error");
            let width: u32 = Result::expect(deminsions.next().unwrap().parse(), "error");
            let height: u32 = Result::expect(deminsions.next().unwrap().parse(),"error");

            total_wrapping += calc_surface_area(&length, &width, &height);
            total_ribbon += calc_ribbon(&length, &width, &height);
        }
        println!("Total amount of sq ft of wrapping paper: {}",total_wrapping);
        println!("Total amount of ribbon required: {}",total_ribbon);
    }
}

fn calc_surface_area(l: &u32, w: &u32, h: &u32) -> u32 {
    let mut smallest = vec![l, w, h];
    smallest.sort();

    let smallest = smallest[0] * smallest[1];

    return (2*l*w) + (2*w*h) + (2*h*l) + smallest
}

fn calc_ribbon(l: &u32, w: &u32, h: &u32) -> u32 {
    let mut smallest = vec![l, w, h];
    smallest.sort();

    (smallest[0]*2 + smallest[1]*2) + (l*w*h)
}
