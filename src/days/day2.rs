use std::fs;

pub fn solution() {
    let file = read_file(String::from("inputs/input2.txt"));

    println!("Solution for first part: {}", solve1(file.clone()));
    println!("Solution for second part: {}", solve2(file.clone()));
}

fn read_file(path: String) -> Vec<Vec<i32>> {
    let mut vec: Vec<Vec<i32>> = vec![vec![]];

    let contents = fs::read_to_string(path).expect("no file");

    let mut i: usize = 0;

    for part in contents.lines() {
        if !part.is_empty() {
            vec.push(vec![]);

            for num in part.split_whitespace() {
                let parse = num.parse::<i32>();

                match parse {
                    Ok(data) => vec[i].push(data),
                    Err(..) => {}
                }
            }
            i += 1;
        }
    }
    vec
}

fn solve1(vec: Vec<Vec<i32>>) -> i32 {
    let mut safe: i32 = vec.len().try_into().unwrap();
    let mut increasing: bool;
    let mut diff: i32;

    for vector in vec {
        if vector.len() == 0 {
            safe -= 1;
            continue;
        }

        increasing = false;

        if vector[0] < vector[1] {
            increasing = true
        }

        for i in 1..vector.len() {
            diff = (vector[i - 1] - vector[i]).abs();

            if ((vector[i - 1] < vector[i]) != increasing) || diff < 1 || diff > 3 {
                safe -= 1;

                break;
            }
        }
    }
    safe
}

fn solve2(vec: Vec<Vec<i32>>) -> i32 {
    let mut safe: i32 = vec.len().try_into().unwrap();
    let mut increasing: bool;
    let mut diff: i32;
    let mut errors: u32;
    let mut i: usize;
    let mut removed_first: bool;
    let mut temp: i32;

    for mut vector in vec.clone() {
        errors = 0;

        if vector.len() == 0 {
            continue;
        }

        increasing = false;

        if vector[0] < vector[1] {
            increasing = true
        }

        removed_first = false;
        temp = 0;
        i = 1;
        let asd: bool = true;

        while i < vector.len() {
            diff = (vector[i - 1] - vector[i]).abs();
            if ((vector[i - 1] < vector[i]) != increasing) || diff < 1 || diff > 3 {
                if i == 1 && !removed_first && asd {
                    errors += 1;
                    i = 1;
                    removed_first = true;
                    temp = vector.clone()[i - 1];
                    vector.remove(i - 1);
                    continue;
                } else if i == 1 && removed_first && asd {
                    i = 1;
                    vector.remove(0);
                    vector.insert(0, temp);
                } else {
                    vector.remove(i);

                    errors += 1;
                    i = 1;
                }
            }

            i += 1;
        }

        if errors >= 2 {
            safe -= 1;
        }
    }
    safe
}
