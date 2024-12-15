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
            Ps.push(Position::new(a[0], a[1]));
        }
    }

    println!("ingested");

    for i in 0..As.len() {
        if let Some(cost) = optimise(&As.pop().unwrap(), &Bs.pop().unwrap(), &Ps.pop().unwrap()) {
            total += cost;
        }
    }

    println!("{total}");
}

fn optimise(A: &ControlInput, B: &ControlInput, P: &Position) -> Option<usize> {
    let mut combos: Vec<(usize, usize)> = vec!();
    let mut alpha = 0;
    loop {
        let target: Position;
        if let Some(retval) = P.subtract(&A.number_of_times(alpha)) {
            target = retval;
        } else {
            println!("subtraction negative");
            break;
        }
        
        if let Some(beta) = B.does_it_work(&target) {
            combos.push((alpha, beta));
        }
        if target.x < 0 || target.y < 0 {
            break;
        }
        alpha += 1;
    }
    if combos.len() == 0 {
        return None;
    }
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
        assert_eq!(Some(280), optimise(&A, &B, &P));
    }
    
    #[test]
    fn test_case_2() {
        let A = ControlInput::new(26, 66);
        let B = ControlInput::new(67, 21);
        let P = Position::new(12748, 12176);
        assert_eq!(None, optimise(&A, &B, &P));
    }

    #[test]
    fn test_case_3() {
        let A = ControlInput::new(17, 86);
        let B = ControlInput::new(84, 37);
        let P = Position::new(7870, 6450);
        assert_eq!(Some(200), optimise(&A, &B, &P));
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