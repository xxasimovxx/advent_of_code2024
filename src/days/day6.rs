use std::fs;
#[derive(Clone, PartialEq)]
enum Cell {
    Empty,
    Visited,
    Obstacle,
}

enum Direction {
    Up,
    Left,
    Right,
    Down,
}

enum Guard {
    VistedNew,
    VistedOld,
    RanAway,
}
pub fn solution() {
    let mut pos: (usize, usize) = (0, 0);
    let file = read_file(String::from("inputs/input6.txt"), &mut pos);

    println!("{:?}", solve1(file.clone(), &mut pos));
    //println!("{:?}", solve2(file.clone(), &relations));
}

fn read_file(path: String, pos: &mut (usize, usize)) -> Vec<Vec<Cell>> {
    let mut vec: Vec<Vec<Cell>> = vec![];
    let mut row: usize = 0;
    let mut column: usize;

    let contents = fs::read_to_string(path).expect("no file");
    for part in contents.lines() {
        column = 0;
        vec.push(vec![]);
        for c in part.chars() {
            match c {
                '#' => vec[row].push(Cell::Obstacle),
                '^' => {
                    pos.0 = row;
                    pos.1 = column;
                    vec[row].push(Cell::Visited);
                }

                _ => vec[row].push(Cell::Empty),
            }

            column += 1;
        }

        row += 1;
    }

    vec
}

fn solve1(mut vec: Vec<Vec<Cell>>, pos: &mut (usize, usize)) -> u32 {
    // 0 row 1 column
    let mut direction = Direction::Up;
    let mut sum: u32 = 1;

    loop {
        match move_pos(&mut vec, &mut direction, pos) {
            Guard::RanAway => return sum,
            Guard::VistedNew => sum += 1,
            Guard::VistedOld => continue,
        }
    }
}

fn move_pos(
    vec: &mut Vec<Vec<Cell>>,
    direction: &mut Direction,
    pos: &mut (usize, usize),
) -> Guard {
    match direction {
        Direction::Up => {
            if pos.0 < 1 {
                return Guard::RanAway;
            }

            pos.0 -= 1;
            match vec[pos.0][pos.1] {
                Cell::Obstacle => {
                    pos.0 += 1;
                    *direction = Direction::Right;
                    return Guard::VistedOld;
                }

                Cell::Empty => {
                    vec[pos.0][pos.1] = Cell::Visited;
                    return Guard::VistedNew;
                }
                Cell::Visited => return Guard::VistedOld,
            }
        }
        Direction::Down => {
            if pos.0 > vec.len() - 1 {
                return Guard::RanAway;
            }

            pos.0 += 1;
            match vec[pos.0][pos.1] {
                Cell::Obstacle => {
                    pos.0 -= 1;
                    *direction = Direction::Left;
                    return Guard::VistedOld;
                }

                Cell::Empty => {
                    vec[pos.0][pos.1] = Cell::Visited;
                    return Guard::VistedNew;
                }
                Cell::Visited => return Guard::VistedOld,
            }
        }
        Direction::Left => {
            if pos.1 < 1 {
                return Guard::RanAway;
            }

            pos.1 -= 1;
            match vec[pos.0][pos.1] {
                Cell::Obstacle => {
                    pos.1 += 1;
                    *direction = Direction::Up;
                    return Guard::VistedOld;
                }

                Cell::Empty => {
                    vec[pos.0][pos.1] = Cell::Visited;
                    return Guard::VistedNew;
                }
                Cell::Visited => return Guard::VistedOld,
            }
        }
        Direction::Right => {
            if pos.1 > vec[pos.0].len() - 1 {
                return Guard::RanAway;
            }

            pos.1 += 1;
            match vec[pos.0][pos.1] {
                Cell::Obstacle => {
                    pos.1 -= 1;
                    *direction = Direction::Down;
                    return Guard::VistedOld;
                }

                Cell::Empty => {
                    vec[pos.0][pos.1] = Cell::Visited;
                    return Guard::VistedNew;
                }
                Cell::Visited => return Guard::VistedOld,
            }
        }
    }
}
