use std::fs;
#[allow(dead_code)]

pub fn main() {
    let input_string = fs::read_to_string("inputs/day6").unwrap();
    let grid_str = input_string.lines().map(|line| line.chars().map(|c|c.to_string()).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>();
    let mut direction = Direction::Up;
    let mut visited_places = vec![];
    
    let mut position = (0,0);

    for y in 0..grid_str.len() {
        for x in 0..grid_str[0].len() {
            if grid_str[y][x] == "^"{
                position = (x, y);
            }
        }
    }

    loop {
        println!("{:?}, {:?}", position, direction);
        if !visited_places.contains(&position) {
            visited_places.push(position);
        }
        let increment = direction.increment();
        let mut next_x = (position.0 as isize + increment.0) as usize;
        let mut next_y = (position.1 as isize + increment.1) as usize;
        if next_x >= grid_str[0].len() || next_y >= grid_str.len() {
            break;
        }
        if grid_str[next_y][next_x] == "#" {
            direction.plus_one();
            let increment = direction.increment();
            next_x = (position.0 as isize + increment.0) as usize;
            next_y = (position.1 as isize + increment.1) as usize;
            if next_x >= grid_str[0].len() || next_y >= grid_str.len() {
                break;
            }
        }
        position = (next_x, next_y);
    }

    println!("{}", visited_places.len());
}

#[derive(Debug)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Direction {
    pub fn increment(&self) -> (isize, isize) {
        return match *self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    pub fn plus_one(&mut self) {
        match self {
            Direction::Up => *self = Direction::Right,
            Direction::Right => *self = Direction::Down,
            Direction::Down => *self = Direction::Left,
            Direction::Left => *self = Direction::Up,

        }
    }
}