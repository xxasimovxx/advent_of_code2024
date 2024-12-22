/* Main idea behind this solution is to make relation such that every number has vector of numbers
   that appear after given number for example : "23|32 \n 23|44" : 23 -> [32,44]
*/

use std::collections::HashMap;
use std::fs;

pub fn solution() {
    let mut relations: HashMap<u32, Vec<u32>> = HashMap::new();
    let file = read_file(String::from("inputs/input5.txt"), &mut relations);

    println!("{:?}", solve1(file.clone(), &relations));
    println!("{:?}", solve2(file.clone(), &relations));
}

fn read_file(path: String, relations: &mut HashMap<u32, Vec<u32>>) -> Vec<Vec<u32>> {
    let mut vec: Vec<Vec<u32>> = vec![];

    let contents = fs::read_to_string(path).expect("no file");

    for part in contents.lines() {
        match part.find("|") {
            Some(n) => {
                let parse = (
                    String::from(&part[0..n]).parse::<u32>(),
                    String::from(&part[n + 1..part.len()]).parse::<u32>(),
                );

                match (parse.0, parse.1) {
                    (Ok(num1), Ok(num2)) => {
                        if relations.contains_key(&num1) {
                            relations.get_mut(&num1).unwrap().push(num2);
                        } else {
                            relations.insert(num1, vec![num2.clone()]);
                        }
                    }
                    _ => {}
                };
            }
            None => {}
        };

        match part.find(",") {
            Some(..) => {
                vec.push(part.split(",").map(|x| x.parse::<u32>().unwrap()).collect());
            }
            None => {}
        };
    }
    vec
}

fn solve1(vec: Vec<Vec<u32>>, relations: &HashMap<u32, Vec<u32>>) -> u32 {
    let mut sum = 0;
    //Brute force way
    for part in vec {
        if is_in_vec(relations, part.clone()) {
            continue;
        }

        sum += &part[usize::try_from(part.len() / 2).unwrap()].clone();
    }

    sum
}

fn solve2(vec: Vec<Vec<u32>>, relations: &HashMap<u32, Vec<u32>>) -> u32 {
    let mut sum = 0;

    for mut part in vec {
        if is_in_vec(relations, part.clone()) {
            part = sort_vec(relations, part);
            sum += &part[usize::try_from(part.len() / 2).unwrap()].clone();
        }
    }
    sum
}

fn sort_vec(relations: &HashMap<u32, Vec<u32>>, vec: Vec<u32>) -> Vec<u32> {
    let mut rev_vec = vec;
    let mut start_point: usize = 0;
    let mut insert_point: usize = 0;
    rev_vec.reverse();

    loop {
        if start_point >= rev_vec.len() - 1 {
            break;
        }

        if relations.contains_key(&rev_vec[start_point]) {
            for i in start_point..rev_vec.len() {
                if relations
                    .get(&rev_vec[start_point])
                    .unwrap()
                    .contains(&rev_vec[i])
                    && i > insert_point
                {
                    insert_point = i;
                }
            }
            if insert_point >0 {
                let temp = rev_vec.remove(start_point);
                if insert_point == rev_vec.len() - 1 {
                    rev_vec.push(temp);
                } else {
                    rev_vec.insert(insert_point, temp);
                } 
                insert_point = 0;
            }else{
                start_point += 1;
            }
        } else {
            start_point += 1;
        }
    }

    rev_vec.reverse();
    rev_vec
}
fn is_in_vec(relations: &HashMap<u32, Vec<u32>>, vec: Vec<u32>) -> bool {
    let mut rev_vec: Vec<u32> = vec;
    rev_vec.reverse();
    for i in 0..rev_vec.len() {
        //Maybe should use match but idk
        if relations.contains_key(&rev_vec[i]) {
            for j in relations.get(&rev_vec[i]).unwrap().clone().iter() {
                if rev_vec[i..rev_vec.len()].contains(j) {
                    return true;
                }
            }
        }
    }
    return false;
}
