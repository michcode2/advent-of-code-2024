use std::fs;

pub fn main() {
    let inputs = fs::read_to_string("inputs/day13").unwrap();
    let mut As = vec!();
    let mut Bs = vec!();
    let mut Ps = vec!();

    let mut total = 0;

    for line in inputs.lines() {
        if line.contains("Button A") {
            let relevant = &line[9..];
            let parts = relevant.split(",");
            let a = parts.map(|str| *&str[3..].parse::<isize>().unwrap()).collect::<Vec<isize>>();
            As.push(ControlInput::new(a[0], a[1]));
        } 
        if line.contains("Button B") {
            let relevant = &line[9..];
            let parts = relevant.split(",");
            let a = parts.map(|str| *&str[3..].parse::<isize>().unwrap()).collect::<Vec<isize>>();
            Bs.push(ControlInput::new(a[0], a[1]));
        }
        if line.contains("Prize") {
            let relevant = &line[6..];
            let parts = relevant.split(",");
            let a = parts.map(|str| *&str[3..].parse::<usize>().unwrap()).collect::<Vec<usize>>();
            Ps.push(Position::new(10000000000000 + a[0], 10000000000000 + a[1]));

            
        }
    }

    println!("ingested");

    for i in 0..As.len() {
        println!("{}",1000*i/As.len());
        if let Some(cost) = optimise(&As.pop().unwrap(), &Bs.pop().unwrap(), &Ps.pop().unwrap()) {
            total += cost;
        }
    }

    println!("{total}");
}

fn optimise(A: &ControlInput, B: &ControlInput, P: &Position) -> Option<usize> {
    let mut combos: Vec<(usize, usize)> = vec!();
    let mut alpha = 0;
    let tolerance = 1000;

    let alpha_est = A.detection(P.x - tolerance, P.y - tolerance);
    
    return Some(combos[0].0 * 3 + combos[0].1);
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_case_1() {
        let A = ControlInput::new(94, 34);
        let B = ControlInput::new(22, 67);
        let P = Position::new(8400, 5400);
        optimise(&A, &B, &P);
    }
    
    #[test]
    fn test_case_2() {
        let A = ControlInput::new(26, 66);
        let B = ControlInput::new(67, 21);
        let P = Position::new(12748, 12176);
        optimise(&A, &B, &P);
    }

    #[test]
    fn test_case_3() {
        let A = ControlInput::new(17, 86);
        let B = ControlInput::new(84, 37);
        let P = Position::new(7870, 6450);
        optimise(&A, &B, &P);
    }
}

struct ControlInput {
    dx: isize, 
    dy: isize,
}

impl ControlInput {
    fn new(dx: isize, dy: isize) -> ControlInput {
        ControlInput{dx, dy}
    }
    fn number_of_times(&self, presses: usize) -> Position {
        Position::new(self.dx as usize * presses, self.dy as usize * presses)
    }

    fn does_it_work(&self, target_pos: &Position) -> Option<usize> {
        let num_x = target_pos.x as f64 / self.dx as f64;
        let num_y = target_pos.y as f64 / self.dy as f64;

        if num_x == num_y && num_x.round() - num_x < 1e6 {
            return Some(num_x as usize);
        }

        return None;
    }

    fn detection(&self, x_trig: usize, y_trig: usize) -> (usize, usize) {
        let num_x = x_trig as f64 / self.dx as f64;
        let num_y = y_trig as f64 / self.dy as f64;
        return (num_x as usize, num_y as usize);
    }
}

struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn subtract(&self, other: &Position) -> Option<Position> {

        if other.x > self.x || other.y > self.y {
            return None;
        }

        Some(Position {
            x: self.x - other.x,
            y: self.y - other.y
        })
    }
    fn new(x: usize, y: usize) -> Position {
        Position{x, y}
    }
}