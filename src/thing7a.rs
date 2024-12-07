use std::fs;
use rand::prelude::*;

pub fn main() {
    let input_string = fs::read_to_string("inputs/day7").unwrap();
    let all_strings: Vec<Vec<Vec<&str>>> = input_string.lines()
        .map(|line| { 
            line.split(":")
            .map(|line_element| line_element.split(" ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>()})
    .collect();
    
    let mut all_numbers = vec![];
    for equation in all_strings {
        let mut temp_vec = vec![];
        equation[1].clone().into_iter().filter(|s| s.len() > 0).for_each(|s| temp_vec.push(s.parse::<usize>().unwrap()));
        temp_vec.push(equation[0][0].parse::<usize>().unwrap());
        all_numbers.push(temp_vec);
    }
    let mut outer_total = 0;
    for numbers in all_numbers {
        if let Some(number) = does_line_work(numbers) {
            outer_total += number;
        }
    }
    println!("{}", outer_total);
}

fn does_line_work(mut input: Vec<usize>) -> Option<usize> {
    let target = input.pop().unwrap();
    let mut attempted: Vec<Vec<bool>> = vec![];
    let mut current_attempt: Vec<bool> = vec![];
    loop {
        if attempted.len() == 2_usize.pow((input.len() - 1) as u32) {
            return None;
        }
        loop {
            current_attempt = vec![];
            for i in 0..input.len() - 1{
                current_attempt.push(rand::random());
            }
            //println!("{:?}\n{:?}", current_attempt, attempted);
            if !attempted.contains(&current_attempt) && current_attempt.len() > 0 {
                attempted.push(current_attempt.clone());
                break;
            }
        }
        let mut total = input[0];
        //println!("out of the loop");
        for i in 1..input.len() {
            if current_attempt[i - 1] {
                total += input[i];
            } else {
                total *= input[i];
            }
        }
        //println!("{}", total);
        if total == target {
            return Some(total);
        }
    }
}