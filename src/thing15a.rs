use std::fs;

pub fn main() {
    let (mut board, mut robot) = import_board("inputs/day15a");
    let commands = import_commands("inputs/day15b");
    for command in commands {
        let next_x = (robot.x as isize + command.0) as usize;
        let next_y = (robot.y as isize + command.1) as usize;
        display_board(&board);
        println!("{}, {}, {:?}", robot.x, robot.y, command);
        match board[next_y][next_x] {
            Point::Nowt => {
                board[robot.y][robot.x] = Point::Nowt;
                robot.x = next_x;
                robot.y = next_y;
                board[robot.y][robot.x] = Point::Robot;
            },
            Point::Wall => (),
            Point::Box => {
                if move_box(&mut board, (next_x, next_y), command) {
                    board[robot.y][robot.x] = Point::Nowt;
                    robot.x = next_x;
                    robot.y = next_y;
                    board[robot.y][robot.x] = Point::Robot;
                }
            },
            Point::Robot => panic!("multiple robots"),
        }
    }

    let mut total = 0;

    for j in 0..board.len() {
        for i in 0..board[j].len() {
            if board[j][i] == Point::Box {
                total += 100 * j + i;
            }
        }
    }
    println!("{}", total);
}

fn move_box(grid: &mut Vec<Vec<Point>>, current_point: (usize, usize), direction: (isize, isize)) -> bool {
    let next_point_x = (current_point.0 as isize + direction.0) as usize;
    let next_point_y = (current_point.1 as isize + direction.1) as usize;
    match grid[next_point_y][next_point_x] {
        Point::Nowt => {
            grid[next_point_y][next_point_x] = Point::Box;
            return true;
        }
        Point::Box => return move_box(grid, (next_point_x, next_point_y), direction),
        Point::Wall => return false,
        Point::Robot => panic!(),
    }
}

fn import_board(path: &str) -> (Vec<Vec<Point>>, Robot) {
    let input_string = fs::read_to_string(path).unwrap();
    let mut board: Vec<Vec<Point>> = vec![];
    let mut robot = Robot{x:0, y:0};
    let all_lines = input_string.lines().collect::<Vec<&str>>();
    for line_index in 0..all_lines.len() {
        board.push(vec![]);
        for c in all_lines[line_index].chars() {
            match c.to_string().as_str() {
                "#" => board[line_index].push(Point::Wall),
                "@" => {
                    board[line_index].push(Point::Robot);
                    robot = Robot{
                        x: board[line_index].len() - 1,
                        y: board.len() - 1,
                    }
                },
                "O" => board[line_index].push(Point::Box),
                "." => board[line_index].push(Point::Nowt),
                _ => panic!(),
            }
        }
    }
    (board, robot)
}

fn import_commands(path: &str) -> Vec<(isize, isize)> {
    let input_string = fs::read_to_string(path).unwrap();
    let mut output_vec = vec![];
    for element in input_string.chars() {
        match element.to_string().as_str() {
            "^" => output_vec.push((0, -1)),
            ">" => output_vec.push((1, 0)),
            "v" => output_vec.push((0, 1)),
            "<" => output_vec.push((-1, 0)), 
            _ => panic!(),
        }
    }
    output_vec
}

fn display_board(board: &Vec<Vec<Point>>) {
    for j in 0..board.len() {
        for i in 0..board[j].len() {
            match board[j][i]{
                Point::Robot => print!("@"),
                Point::Box => print!("O"), 
                Point::Nowt => print!(" "),
                Point::Wall => print!("#"),
            }
        }
        println!();
    }
}

struct Robot {
    x: usize,
    y: usize,
}

#[derive(PartialEq)]
enum Point {
    Robot, 
    Box,
    Wall,
    Nowt,
}