use std::fs;
// 774, 705 671, 613, 568, 502, 465, 401, 362, 
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

    for i in 0..50000 {
        find_entropy(i, guards.clone(), file.clone());
    }

}

fn find_entropy(num_steps: isize, mut guards: Vec<Guard>, file: String) {
    let grid_x: isize = 101;
    let grid_y: isize = 103;

    let num_buckets = 6;
    let mut buckets: Vec<Vec<usize>> = vec![];

    for i in 0..num_buckets + 1 {
        buckets.push(vec![]);
        for _ in 0..num_buckets + 1{
            buckets[i].push(0);
        }
    }

    let watford_gap = grid_x as usize/num_buckets;
    let pennines = grid_y as usize /num_buckets;

    for i in 0..guards.len() {
        guards[i].take_steps(num_steps, (grid_x, grid_y));
        let pos = &guards[i].position;
        let bucket_down = (pos.y as usize / watford_gap) as usize;
        let bucket_side = (pos.x as usize / pennines) as usize;
        buckets[bucket_down][bucket_side] += 1;
    }

    let mut num_bots = file.lines().count();
    let num_buckets = num_buckets * num_buckets;
    let bots_per_bucket = num_bots/num_buckets;
    let mut entropy = 0;
    for j in 0..buckets.len() {
        for i in 0..buckets[j].len() {
            entropy += usize::abs_diff(bots_per_bucket, buckets[j][i]);
        }
    }

    if entropy > 800 || entropy < 200{
        display(&guards, (grid_x as usize, grid_y as usize), num_steps);
    }
}


fn display(guards: &Vec<Guard>, grid_dim: (usize, usize), i: isize) {
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

    let mut output = String::new();
    for j in 0..grid_dim.0{
        for i in 0..grid_dim.1{
            if grid[i][j] > 0 {
                output += &grid[i][j].to_string();
            } else {
                output+=".";
            }
        }
        output+="\n";
    }
    fs::write(format!("cache/num{i}"), output).expect("Unable to write file");

}

#[derive(Clone)]
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