use std::fs;

pub fn main() {
    let input_string = fs::read_to_string("inputs/day7").unwrap();
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
}

struct Antenna{
    x: usize,
    y: usize,
    freq: String,
}