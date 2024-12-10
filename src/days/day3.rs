use regex::Regex;
use std::fs;

pub fn solution() {
    let file = read_file(String::from("inputs/input3.txt"));
    println!("{:?}", solve1(file.clone()));
    println!("{:?}", solve2(file.clone()));
}

fn solve1(string: String) -> i32 {
    let mut sum = 0;
    let mut offset: usize;
    let mut i: usize = 0;
    let re = Regex::new(r"mul\(\b([1-9]\d{0,3}),\s*([1-9]\d{0,3})\b\)").unwrap();

    for cap in re.captures_iter(&string) {
        let num1 = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let num2 = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
        sum += num1 * num2;
    }

    sum
}

fn solve2(string: String) -> i32 {
    let mut sum = 0;
    let mut offset: usize;
    let mut i: usize = 0;
    let re = Regex::new(r"mul\(\b([1-9]\d{0,3}),\s*([1-9]\d{0,3})\b\)").unwrap();

    for cap in re.captures_iter(&string) {
        let num1 = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let num2 = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
        sum += num1 * num2;
    }

    sum
}
fn read_file(path: String) -> String {
    fs::read_to_string(path).expect("no file")
}
