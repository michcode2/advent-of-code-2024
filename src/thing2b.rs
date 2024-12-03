use std::fs;
#[allow(unused_parens)]
#[allow(dead_code)]

pub fn main(){
    let thing = fs::read_to_string("inputs/day2").unwrap();
    let lines = thing.lines();
    let mut safe = 0;
    for line in lines.clone() {
        let numbers_str = line.split(" ");
        let numbers = numbers_str.map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        if parse_line_b(numbers) {
            safe += 1;
        }
    }
    println!("{}", safe);
}

fn parse_line_b(input: Vec<usize>) -> bool {
    if parse_line_a(input.clone()) {
        return true;
    }
    for i in 0..input.len(){
        let mut temp = input.clone();
        temp.remove(i);
        if parse_line_a(temp) {
            return true;
        }
    } 
    return false;
}

fn parse_line_a(input: Vec<usize>) -> bool {
    check_gradient(&input) && check_same_direction(&input)
}

fn check_same_direction(input: &Vec<usize>) -> bool {
    let increasing = input[0] < input[1];
    for i in 1..input.len() {
        if (input[i-1] < input[i]) ^ increasing {
            return false;
        }
    }
    true
}

fn check_gradient(input: &Vec<usize>) -> bool {
    for i in 1..input.len() {
        let gradient = usize::abs_diff(input[i], input[i-1]);
        if (gradient > 3) || (gradient < 1){
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn line_1() {
        let numbers = vec![7, 6, 4, 2, 1];
        assert_eq!(true, parse_line_a(numbers.clone()));
        assert_eq!(parse_line_b(numbers), true);
    }
    #[test]
    fn line_2() {
        let numbers = vec![1, 2, 7, 8, 9];
        assert_eq!(false, parse_line_a(numbers.clone()));
        assert_eq!(parse_line_b(numbers), false);
    }
    #[test]
    fn line_3() {
        let numbers = vec![9, 7, 6, 2, 1];
        assert_eq!(false, parse_line_a(numbers.clone()));
        assert_eq!(parse_line_b(numbers), false);
    }
    #[test]
    fn line_4() {
        let numbers = vec![1, 3, 2, 4, 5];
        assert_eq!(false, parse_line_a(numbers.clone()));
        assert_eq!(parse_line_b(numbers), true);
    }
    #[test]
    fn line_5() {
        let numbers = vec![8, 6, 4, 4, 1];
        assert_eq!(false, parse_line_a(numbers.clone()));
        assert_eq!(parse_line_b(numbers), true);
    }
    #[test]
    fn line_6() {
        let numbers = vec![1, 3, 6, 7, 9];
        assert_eq!(true, parse_line_a(numbers.clone()));
        assert_eq!(parse_line_b(numbers), true);
    }
}