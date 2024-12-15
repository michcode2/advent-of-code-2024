use std::fs;

pub fn main() {
    let file = fs::read_to_string("inputs/day14").unwrap();
    let mut guards = vec!();
    for line in file.lines() {
        let things = line.split("=").collect::<Vec<&str>>();
        let vels = things[2].split(",").map(|n| n.parse::<isize>().unwrap()).collect::<Vec<isize>>();
        let pos = things[1].split(" ").next().unwrap().split(",").map(|n| n.parse::<isize>().unwrap()).collect::<Vec<isize>>();
        guards.push(Guard{
            velocity: Vec2 {
                x: vels[0],
                y: vels[1],
            },
            position: Vec2 { 
                x: pos[0], 
                y: pos[1],
            }
        }

        )
    }

    let grid_x = 101;
    let grid_y = 103;

    let mut nw = 0;
    let mut ne = 0;
    let mut sw = 0;
    let mut se = 0;

    let watford_junction = grid_x/2;
    let pennines = grid_y/2;

    for i in 0..guards.len() {
        guards[i].take_steps(100, (grid_x, grid_y));
        let pos = &guards[i].position;
        if pos.x < watford_junction && pos.y < pennines {
            nw += 1;
        }
        if pos.x > watford_junction && pos.y < pennines {
            sw += 1;
        }
        if pos.x < watford_junction && pos.y > pennines {
            ne += 1;
        }
        if pos.x > watford_junction && pos.y > pennines {
            se += 1;
        }
    }

    println!("{}", nw*ne*sw*se);

}

fn display(guards: &Vec<Guard>, grid_dim: (usize, usize)) {
    let mut grid = vec![];
    for j in 0..grid_dim.1{
        grid.push(vec![]);
        for i in 0..grid_dim.0{
            grid[j].push(0);
        }
    }

    for g in guards {
        grid[g.position.y as usize][g.position.x as usize] += 1;
    }

    for j in 0..grid_dim.1{
        for i in 0..grid_dim.0{
            if grid[j][i] > 0 {
                print!("{}", grid[j][i]);
            } else {
                print!(".");
            }
        }
        println!();
    }

}

struct Guard {
    velocity: Vec2,
    position: Vec2,
}

impl Guard {
    fn take_steps(&mut self, num_steps: isize, grid_size: (isize, isize)) {
        let new_position = self.position + (self.velocity * num_steps);
        let loops_x = new_position.x / grid_size.0;
        let loops_y = new_position.y / grid_size.1;
        let mut x = new_position.x - (loops_x * grid_size.0);
        let mut y = new_position.y - (loops_y * grid_size.1);
        if x < 0 {
            x += grid_size.0;
        }
        
        if y < 0 {
            y += grid_size.1;
        }
        self.position = Vec2 {
            x,
            y,
        }
    }
}

#[derive(Clone, Copy)]
struct Vec2 {
    x: isize,
    y: isize,
}

impl std::ops::Mul<isize> for Vec2 {
    type Output = Self;
    fn mul(self, other: isize) -> Self {
        Vec2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl std::ops::Add for Vec2{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec2 { 
            x: self.x + other.x, 
            y: self.y + other.y, 
        }
    }
}