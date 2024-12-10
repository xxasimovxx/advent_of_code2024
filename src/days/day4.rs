
use std::fs;

pub fn solution() {
    let file = read_file(String::from("inputs/input3.txt"));
    println!("{:?}", file.clone());
//    println!("{:?}", solve2(file.clone()));
}

fn read_file(path: String) -> Vec<Vec<i32>> {
    let mut vec: Vec<Vec<i32>> = vec![vec![]];

    let contents = fs::read_to_string(path).expect("no file");

    vec
}
