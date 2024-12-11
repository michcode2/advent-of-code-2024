use std::fs;

pub fn main(){
    let mut input: Vec<_> = fs::read_to_string("inputs/day11").unwrap().split(" ").map(|n| n.parse::<usize>().unwrap()).collect();
    for i in 0..25 {
        println!("{i}");
        input = blink(&input);
    }
    println!("{}", input.len());
}

fn blink(input: &Vec<usize>) -> Vec<usize> {
    let mut output = vec!();
    let parsed:Vec<_> = input.iter().map(|&x| parse_stone(x)).collect();

    for mut v in parsed {
        v.reverse();
        while v.len() > 0 {
            output.push(v.pop().unwrap());
        }
    }
    output
}

fn parse_stone(value: usize) -> Vec<usize> {
    if value == 0 {
        return vec!(1);
    }
    let as_str = value.to_string();
    if as_str.len() % 2 == 0 {
        let half_len = as_str.len() / 2;
        let num_1 = as_str[..half_len].parse::<usize>().unwrap();
        let num_2 = as_str[half_len..].parse::<usize>().unwrap();
        return vec!(num_1, num_2);
    }
    return vec!(value * 2024);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parse_stone_works() {
        assert_eq!(parse_stone(125), vec![253000]);
        assert_eq!(parse_stone(17), vec![1, 7]);
    }

    #[test]
    fn blink_works() {
        let input = vec![0, 1, 10, 99, 999];
        let output = vec![1, 2024, 1, 0, 9, 9, 2021976];
        assert_eq!(blink(&input), output);
    }

    #[test]
    fn repeated_blink() {
        let mut input = vec![125, 17];
        let output = vec![2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3, 2];
        for _ in 0..6 {
            input = blink(&input);
        }
        assert_eq!(input, output);
    }
}