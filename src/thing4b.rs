use std::fs;
#[allow(dead_code)]

pub fn main(){
    let input_string = fs::read_to_string("inputs/day4").unwrap();
    let grid: Vec<Vec<char>> = input_string.lines().map(|line| line.chars().collect()).collect();
    let target_string:Vec<char> = "XMAS".chars().collect();
    let mut total = 0;
    for x in 1..grid.len()-1 {
        for y in 1..grid[0].len()-1 {
            total += check_here(x, y, &target_string, &grid, 0);
        }
    }
    println!("{}", total);
}

fn check_here(x: usize, y: usize, target: &Vec<char>, grid: &Vec<Vec<char>>, index: usize) -> usize {
    if "A" != grid[x][y].to_string() {
        return 0;
    }

    println!("{},{}", x, y);

    let tr = grid[x+1][y+1].to_string();
    let bl = grid[x-1][y-1].to_string();
    let tl = grid[x-1][y+1].to_string();
    let br = grid[x+1][y-1].to_string();

    if tr == "S" && bl == "M" || tr == "M" && bl == "S" {
        if tl == "S" && br == "M" || tl == "M" && br == "S" {
            return 1;
        }
    }
    return 0;
}