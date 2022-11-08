use std::fs;
use std::io;

fn main() {
    let no_prob = fs::File::open("./test_dir/short_text.txt").expect("you will not see this error");
    println!("no_prob file handle: {:?}", no_prob);
    let is_dir = fs::File::open("./test_dir/dir").expect("Is a directory");
    println!("is_dir file handle: {:?}", is_dir);
    let mut str = String::new();
    str = fs::read_to_string("./test_dir/dir").expect("Is a directory");
    println!("str: {:?}", str);
}
