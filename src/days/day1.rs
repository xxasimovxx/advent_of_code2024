use std::fs;

pub fn solution(){

    let file = read_file(String::from("inputs/input1_1.txt"));

    println!("{}", solve(file));

}

fn read_file(path: String) -> (Vec<u32>,Vec<u32>) {

    let mut vec_left: Vec<u32> = vec![];
    let mut vec_right: Vec<u32> = vec![];
    
    let contents = fs::read_to_string(path)
        .expect("no file");

    let mut numbers : Vec<u32> = vec![];

    let mut parts = contents.split_whitespace().map(|x| x.parse::<u32>());
    
    loop{
    match parts.next() {
        Some(Ok(part)) => numbers.push(part),
        None => break,
        _ => {}

    }}
    println!("{:?}", numbers);

    for i in 0..(numbers.len())/2{
        vec_left.push(numbers[i * 2]);
        vec_right.push(numbers[i * 2 + 1]);

    }

    vec_left.sort();
    vec_right.sort();

    (vec_left,vec_right)

}

fn solve((vec_left, vec_right): (Vec<u32>,Vec<u32>)) -> u32{

    let mut sum: u32 = 0;
    /*
    let mut range: (usize, usize) = (0,0); 
    let mut range1: (u32, u32) ; 
    let mut change: bool;
    */
    
    for i in 0..vec_left.len(){
        sum += vec_left[i].abs_diff(vec_right[i]);
    }
/*
    loop{

        change = false;
        println!("{}", vec.len());

        for i in 0..vec.len()-1{

            range1 = (vec[range.0].0,vec[range.1].1); 

            let (x,y) = &mut vec[i];
            if *x < range1.0{

                range.0 = i;
                range1.0 = *x;
                change = true;

            }

            if *y < range1.1{

                range.1 = i;
                range1.1 = *y;
                change = true;

            }
        }

        if !change{
            break;

        }else{

            sum += vec[range.0].0.abs_diff(vec[range.1].1);
            vec[range.0].0 = u32::MAX;
            vec[range.1].1 = u32::MAX;

        }
    }
*/
    sum

}

