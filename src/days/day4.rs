use std::fs;

pub fn solution() {
    let file = read_file(String::from("inputs/input4.txt"));
    println!("{:?}", solve1(file.clone()));
    println!("{:?}", solve2(file.clone()));
}

fn read_file(path: String) -> Vec<Vec<char>> {
    let mut vec: Vec<Vec<char>> = vec![];
    let contents = fs::read_to_string(path).expect("no file");

    for str in contents.lines() {
        vec.push(str.chars().collect());
    }

    vec
}

fn solve1(string: Vec<Vec<char>>) -> u32 {
    let mut sum: u32 = 0;

    for i in 0..string.len() {
        for j in 0..string[i].len() {
            if string[i][j] == 'X' {
                sum += xmas_count(&string, i, j);
            }
        }
    }

    sum
}

fn xmas_count(string: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    let mut sum: u32 = 0;
    //Horizontal
    let mut temp: String = String::from("");
    if j + 3 < string[i].len() {
        temp.push(string[i][j]);
        temp.push(string[i][j + 1]);
        temp.push(string[i][j + 2]);
        temp.push(string[i][j + 3]);

        if temp == String::from("XMAS") {
            sum += 1;
        }

        temp.clear();
    }

    if j > 2 {
        temp.push(string[i][j]);
        temp.push(string[i][j - 1]);
        temp.push(string[i][j - 2]);
        temp.push(string[i][j - 3]);

        if temp == String::from("XMAS") {
            sum += 1;
        }

        temp.clear();
    }

    //Vertical
    if i + 4 < string.len() {
        temp.push(string[i][j]);
        temp.push(string[i + 1][j]);
        temp.push(string[i + 2][j]);
        temp.push(string[i + 3][j]);

        if temp == String::from("XMAS") {
            sum += 1;
        }

        temp.clear();
    }

    if i > 2 {
        temp.push(string[i][j]);
        temp.push(string[i - 1][j]);
        temp.push(string[i - 2][j]);
        temp.push(string[i - 3][j]);

        if temp == String::from("XMAS") {
            sum += 1;
        }

        temp.clear();
    }
    //Diagonal

    if i + 4 < string.len() && j + 3 < string[i].len() {
        temp.push(string[i][j]);
        temp.push(string[i + 1][j + 1]);
        temp.push(string[i + 2][j + 2]);
        temp.push(string[i + 3][j + 3]);

        if temp == String::from("XMAS") {
            sum += 1;
        }

        temp.clear();
    }

    if i > 2 && j + 3 < string[i].len() {
        temp.push(string[i][j]);
        temp.push(string[i - 1][j + 1]);
        temp.push(string[i - 2][j + 2]);
        temp.push(string[i - 3][j + 3]);

        if temp == String::from("XMAS") {
            sum += 1;
        }

        temp.clear();
    }

    if i + 4 < string.len() && j > 2 {
        temp.push(string[i][j]);
        temp.push(string[i + 1][j - 1]);
        temp.push(string[i + 2][j - 2]);
        temp.push(string[i + 3][j - 3]);

        if temp == String::from("XMAS") {
            sum += 1;
        }

        temp.clear();
    }

    if i > 2 && j > 2 {
        temp.push(string[i][j]);
        temp.push(string[i - 1][j - 1]);
        temp.push(string[i - 2][j - 2]);
        temp.push(string[i - 3][j - 3]);

        if temp == String::from("XMAS") {
            sum += 1;
        }

        temp.clear();
    }

    sum
}

fn solve2(string: Vec<Vec<char>>) -> u32 {
    let mut sum: u32 = 0;

    for i in 0..string.len() {
        for j in 0..string[i].len() {
            if string[i][j] == 'A' {
                sum += x_mas_count(&string, i, j);
            }
        }
    }

    sum
}

fn x_mas_count(string: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    let mut sum: u32 = 0;
    let mut temp: Vec<char> = vec![];

    if i > 0 && j > 0 && i + 2 < string.len() && j + 1 < string[i].len() {
        temp.push(string[i - 1][j - 1]); //LU
        temp.push(string[i + 1][j + 1]); //RD
        temp.push(string[i - 1][j + 1]); //RU
        temp.push(string[i + 1][j - 1]); //LD

        for &chr in &temp {
            if !(chr == 'M' || chr == 'S') {
                sum = 0;
                return sum;
            }
        }

        if temp[0] != temp[1] && temp[2] != temp[3] {
            sum = 1;
        }
    }

    sum
}
