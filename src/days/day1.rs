use std::collections::HashMap;
use std::fs;

pub fn solution() {
    let file = read_file(String::from("inputs/input1.txt"));

    println!("Solution for first part: {}", solve1(file.clone()));
    println!("Solution for second part: {}", solve2(file.clone()));
}

fn read_file(path: String) -> (Vec<u32>, Vec<u32>) {
    let mut vec_left: Vec<u32> = vec![];
    let mut vec_right: Vec<u32> = vec![];

    let contents = fs::read_to_string(path).expect("no file");

    let mut numbers: Vec<u32> = vec![];

    let mut parts = contents.split_whitespace().map(|x| x.parse::<u32>());

    loop {
        match parts.next() {
            Some(Ok(part)) => numbers.push(part),
            None => break,
            _ => {}
        }
    }

    for i in 0..(numbers.len()) / 2 {
        vec_left.push(numbers[i * 2]);
        vec_right.push(numbers[i * 2 + 1]);
    }

    vec_left.sort();
    vec_right.sort();

    (vec_left, vec_right)
}

fn solve1((vec_left, vec_right): (Vec<u32>, Vec<u32>)) -> u32 {
    let mut sum: u32 = 0;

    for i in 0..vec_left.len() {
        sum += vec_left[i].abs_diff(vec_right[i]);
    }

    sum
}

fn solve2((vec_left, vec_right): (Vec<u32>, Vec<u32>)) -> u32 {
    let mut sum: u32 = 0;
    let mut amount: u32;

    let mut appear: HashMap<u32, u32> = HashMap::new();

    for num in &vec_left {
        if appear.get(num).copied().unwrap_or(0) == 0 {
            amount = u32::try_from(vec_right.iter().filter(|&x| x == num).count()).unwrap();
            appear.insert(*num, amount);
            sum += num * amount;
        } else {
            sum += num * appear.get(num).copied().unwrap_or(0);
        }
    }

    sum
}
