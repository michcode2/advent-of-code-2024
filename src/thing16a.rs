use std::fs;

pub fn main() {
    let (board, start, end) = import_board("inputs/test");

    let mut completed = vec![];

    investigate(start, &board, &mut completed, 0, Direction::East);
}


fn investigate(point: (usize, usize), board: &Vec<Vec<bool>>, completed: &mut Vec<usize>, cost: usize, facing: Direction) {
    println!("{cost}, {facing:?}, {point:?}");
    let mut options;
    match facing {
        Direction::North => options = [(-1, 0), (0, 1), (1, 0)], //ccw, fwd, cw
        Direction::East => options = [(0, 1), (1, 0), (0, -1)],
        Direction::South => options = [(1, 0), (-1, 0), (-1, 0)],
        Direction::West => options = [(0, -1), (0, -1), (0, 1)],
        _ => panic!(),
    }
    for i in 0..options.len() {
        let x = (point.0 as isize - options[i].0) as usize;
        let y = (point.1 as isize - options[i].1) as usize;
        
        if y > board.len() || x > board[0].len(){
            continue;
        }
        if board[y][x].to_string().as_str() == "#" {
            continue;
        }
        if board[y][x].to_string().as_str() == "E" {
            completed.push(cost);
            continue;
        }
        if i == 0 {
            investigate((x, y), board, completed, cost+1000, facing.clone().anticlockwise());
        } else if i == 2 {
            investigate((x, y), board, completed, cost+1000, facing.clone().clockwise());
        } else {
            investigate((x, y), board, completed, cost+1, facing);
        }
    }
}
fn import_board(path: &str) -> (Vec<Vec<bool>>, (usize, usize), (usize, usize)) {
    let input_string = fs::read_to_string(path).unwrap();
    let mut board = vec![];
    for l in input_string.lines() {
        board.push(l.chars().map(|c| c.to_string().as_str() == "#").collect::<Vec<bool>>());
    }

    let mut start = (0,0);
    let mut end = (0,0);

    for j in 0..board.len() {
        for i in 0..board[0].len() {
            match board[j][i].to_string().as_str() {
                "S" => start = (i, j),
                "E" => end = (i, j),
                _ => (),
            }
        }
    }
    return (board, start, end);
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn clockwise(&self) -> Direction {
        match self{
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,

        }
    }
    fn anticlockwise(&self) -> Direction {
        let a = self.clockwise();
        let b = a.clockwise();
        b.clockwise()
    }
}