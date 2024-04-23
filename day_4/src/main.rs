use md5::{self, Digest, Md5};
//41 m 10s
fn main() {
    let input = String::from("iwrupvqb");
    let mut counter = 1u64;
    let mut five_found = false;
    let mut six_found = false;
    loop {
        let key = format!("{}{}", input,counter);
        let hash = Md5::digest(key);
        if format!("{:x}",hash).starts_with("00000") && !five_found {
            println!("The lowest number is for five leading zeroes is: {}",counter);
            five_found = true;
        }

        if format!("{:x}",hash).starts_with("000000") && !six_found {
            println!("The lowest number is for six leading zeroes is: {}",counter);
            six_found = true;
        }
        counter += 1;
    }
}

