use std::fs;
use std::sync::mpsc;
use threadpool::ThreadPool;

pub fn main() {
    let input_string = fs::read_to_string("inputs/day6").unwrap();
    let grid_str = input_string.lines().map(|line| line.chars().map(|c|c.to_string()).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>();


    let pool = ThreadPool::new(8);
	let (tx, rx) = mpsc::channel();

    for x in 0..grid_str.len() {
        for y in 0..grid_str[0].len() {
            let new_tx = tx.clone();
            let grid_cloned = grid_str.clone();
            pool.execute(move || {
                new_tx.send(does_it_converge(&x, &y, grid_cloned)).unwrap();
            });
        }
    }

    pool.join();
	drop(tx);

    let mut converge_count = 0;
    for message in rx {
        if message {
            converge_count += 1;
        }
    }
    println!("{}", converge_count);
}

fn does_it_converge(box_x: &usize, box_y: &usize, mut grid_str: Vec<Vec<String>>) -> bool {
    let mut direction = Direction::Up;
    let mut visited_places = vec![];
    
    let mut position = (0,0, direction);

    for y in 0..grid_str.len() {
        for x in 0..grid_str[0].len() {
            if grid_str[y][x] == "^"{
                position = (x, y, direction);
            }
        }
    }
    
    if grid_str[*box_y][*box_x] == "." {
        grid_str[*box_y][*box_x] = "#".to_string();
    } else {
        println!("on box or guard");
        return false;
    }

    loop {

        //println!("{:?}", position);
        if visited_places.contains(&(position.0, position.1, direction)) {
            return true;
        }
        visited_places.push((position.0, position.1 , direction));
        let increment = direction.increment();
        let mut next_x = (position.0 as isize + increment.0) as usize;
        let mut next_y = (position.1 as isize + increment.1) as usize;
        if next_x >= grid_str[0].len() || next_y >= grid_str.len() {
            return false;
        }
        if grid_str[next_y][next_x] == "#" {
            direction.plus_one();
            next_x = position.0;
            next_y = position.1;
            // dont step forwards, might be two boxes right next to eachother
        }
        position = (next_x, next_y, direction);
    }
}


#[derive(Debug, PartialEq, Copy, Clone)]
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