use std::fs;

pub fn main() {
    let mut fields = fs::read_to_string("inputs/test").unwrap().lines().map(|l| l.chars().map(|c| c.to_string()).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>();
    start_from_here(&mut fields, 0, 0);
}

fn start_from_here(mut grid: &mut Vec<Vec<String>>, x: usize, y: usize) {
    let mut positions_near_here = vec![];
    let plant = grid[y][x].clone();
    let mut adjacent = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    if plant == "." {
        return;
    }
    let mut things_to_check = vec![];

    for point in adjacent {
        let new_x = (x as isize + point.0) as usize;
        let new_y = (y as isize + point.1) as usize;
        if new_x >= grid[0].len() || new_y >= grid.len() {
            continue;
        } 
        things_to_check.push((new_x, new_y));
    }

    while things_to_check.len() > 0 {
        let next_point = things_to_check.pop().unwrap();
        if grid[next_point.1][next_point.0] == plant {
            positions_near_here.push(next_point);
            for point in adjacent {
                let new_x = (next_point.0 as isize + point.0) as usize;
                let new_y = (next_point.1 as isize + point.1) as usize;
                    if new_x >= grid[0].len() || new_y >= grid.len() {
                        continue;
                    } 
                things_to_check.push((new_x, new_y));
            }
            grid[next_point.1][next_point.0] = String::from(".");
        }
    }
    print!("{}", positions_near_here.len());
}

fn perimeter_of_field(full_field: &Vec<Vec<String>>, crop_patch: &Vec<(usize, usize)>){
    let mut dims = vec![];

    for i in 0..2 {
        let single_direction = crop_patch.iter().map(|point| point.0).collect::<Vec<usize>>();
        let biggest = single_direction.iter().max().unwrap();
        let smallest = single_direction.iter().min().unwrap();
        let dl = biggest - smallest;
        dims.push(dl);
    }
    let perimeter = 2 * (dims[0] + dims[1]);
    let area = dims[0] * dims[1];

    let mut mesh = Mesh {
        perimeter,
        area_field: area,
    };


}

struct Mesh {
    perimeter: usize,
    area_field: usize,
}