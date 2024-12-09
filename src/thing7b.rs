use std::{fs, thread};
use rand::{distributions::Uniform, prelude::*};
use std::sync::mpsc;
#[allow(dead_code)]

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

    println!("read");

    let mut outer_total = 0;
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];
    for numbers in all_numbers.clone() {
        let new_tx = tx.clone();
        handles.push(thread::spawn(move || { 
            if let Some(number) = does_line_work(numbers) {
                new_tx.send(number).unwrap();
            } else {
                new_tx.send(0).unwrap();
            }
        }));
    }
    println!("started threads");
    let mut num_completed = 0.0;
    for handle in handles {
        handle.join().unwrap();
        let message = rx.recv().unwrap();
        outer_total+= message;
        num_completed += 1.0;
        println!("{}", num_completed/(all_numbers.len() as f64));
    }

    println!("{}", outer_total);
}

fn does_line_work(mut input: Vec<usize>) -> Option<usize> {
    let target = input.pop().unwrap();
    let mut attempted: Vec<Vec<Operator>> = vec![];
    let mut current_attempt: Vec<Operator>;
    let mut rng = thread_rng();
    let distribution = Uniform::from(0..=2);
    loop {
        if attempted.len() == 3_usize.pow((input.len() - 1) as u32) {
            return None;
        }
        loop {
            current_attempt = vec![];
            for _ in 0..input.len() - 1{
                match distribution.sample(&mut rng){
                    0 => current_attempt.push(Operator::Add),
                    1 => current_attempt.push(Operator::Times),
                    2 => current_attempt.push(Operator::Concatenate),
                    _ => panic!("sampled from outside the distribution????"),
                }
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
            match current_attempt[i-1] {
                Operator::Add => total += input[i],
                Operator::Times => {
                    if let Some(value) = total.checked_mul(input[i]) {
                        total = value;
                    } else {
                        break;
                    }
                },
                Operator::Concatenate => {
                    let string_1 = total.to_string();
                    let string_2 = input[i].to_string();

                    if let Ok(value) = (string_1 + &string_2).parse::<usize>() {
                        total = value;
                    } else {
                        break;
                    }
                }
            }
        }
        if total == target {
            return Some(total);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    Add,
    Times,
    Concatenate,
}