use std::fs;

pub fn main() {
    let input_string = fs::read_to_string("inputs/day8").unwrap();
    let mut antennas = vec![];
    let lines = input_string.lines().collect::<Vec<&str>>();
    for y in 0..lines.len() {
        let current_line = lines[y].chars().map(|x|x.to_string()).collect::<Vec<String>>();
        for x in 0..current_line.len() {
            if current_line[x].to_string() == "." {
                continue;
            }
            let current_antenna = Antenna {
                x, 
                y,
                freq: current_line[x].to_string(),
            };
            antennas.push(current_antenna);
        }
    }

    let mut antinodes = vec![];

    for i in 0..antennas.len() {
        for j in 0..antennas.len() {
            if i == j {
                continue;
            }
            let ant1 = &antennas[i];
            let ant2 = &antennas[j];
            if ant1.freq != ant2.freq {
                continue;
            }
            if 2*ant1.x < ant2.x {
                continue;
            }
            if 2*ant1.y < ant2.y {
                continue;
            }
            let x_anti = 2*(ant1.x) - ant2.x;
            let y_anti = 2*(ant1.y) - ant2.y;
            
            if x_anti >= lines[0].len() || y_anti >= lines.len() {
                continue;
            }

            if antinodes.contains(&(x_anti, y_anti)) {
                continue;
            }
            else {
                antinodes.push((x_anti, y_anti));
            }
        }
    }

    println!("{:?}", antinodes);
    println!("{}", antinodes.len());
}

struct Antenna{
    x: usize,
    y: usize,
    freq: String,
}