use std::fs;

pub fn main(){
    let input_string = fs::read_to_string("inputs/day4").unwrap();
    let grid: Vec<Vec<char>> = input_string.lines().map(|line| line.chars().collect()).collect();
    let target_string:Vec<char> = "XMAS".chars().collect();
    let mut total = 0;
    for x in 0..grid.len() {
        for y in 0..grid[0].len(){
            for dx in -1..=1 {
                for dy in -1..=1 {
                    //println!("({},{})   ({},{})",x, y, dx, dy);
                    total += check_here(x, y, &target_string, &grid, 0, dx, dy);
                }
            }
        }
    }
    /*for row in grid{
        for char in row {
            print!("{}", char);
        }
        println!();
    }*/
    println!("{}", total);
}

fn check_here(x: usize, y: usize, target: &Vec<char>, grid: &Vec<Vec<char>>, index: usize, dx: isize, dy: isize) -> usize {
    if index == 4 {
        print!("{},{} --- ", x as isize - (4*dx), y as isize -(4*dy));
        println!("{},{}", x as isize - dx, y as isize -dy);
        return 1;
    }

    if index == 3 {
        println!("{},{}", x, y);
    }

    if grid[x][y] == target[index] {
        let next_x = (x as isize + dx) as usize;
        let next_y = (y as isize + dy) as usize;

        if next_x >= grid.len() || next_y >= grid[0].len() {
            if index == 3 {
                return 1;
            }
            let (width, height) = (grid.len(), grid[0].len());
            return 0;
        }
        return check_here(next_x, next_y, target, grid, index + 1, dx, dy);
    }
    return 0;
}