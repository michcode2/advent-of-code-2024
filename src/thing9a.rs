use std::{fmt::{Debug, Display}, fs, ops::Index};

pub fn main() {
    let input_string = fs::read_to_string("inputs/day9").unwrap();
    let ids = get_block_ids(input_string);
    println!("{}", refragment(ids));
}

fn get_block_ids(input: String) -> Vec<String>{
    let mut ids = vec![];
    let mut file = true;
    let mut current_id = 0;
    for num in input.chars() {
        let num_as_int = num.to_string().parse::<usize>().unwrap();
        if file {
            for _ in 0..num_as_int {
                ids.push(current_id.to_string());
            }
            current_id += 1;
        } else {
            for _ in 0..num_as_int {
                ids.push(".".to_string());
            }
        }
        file = !file;
    }
    ids
}

fn refragment(mut input: Vec<String>) -> usize {
    loop {
        let next_dot = input.clone().into_iter().position(|i| i == ".");
        //pretty_print(&input);
        //println!("{:?}", next_dot);
        if let Some(index) = next_dot {
            let mut end_of_vec = input.pop().unwrap();
            while end_of_vec == "." {
                end_of_vec = input.pop().unwrap();
            }
            if index > input.len() {
                println!("fuckass weird edge case");
                input.push(end_of_vec);
                break;
            }
            input[index] = end_of_vec;
        } else {
            break;
        }
    }
    //pretty_print(&input);
    let mut total = 0;
    for i in 0..input.len(){
        total += input[i].parse::<usize>().unwrap() * i;
    }
    total
}

fn pretty_print<T: Display>(input: &Vec<T>) {
    for i in 0..input.len() {
        print!("{},", input[i]);
    }
    println!();
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn compressed_to_bigstring() {
        let input_test = "2333133121414131402".to_string();
        let correct_output = "00...111...2...333.44.5555.6666.777.888899".chars().map(|x| {x.to_string()}).collect::<Vec<String>>();
        assert_eq!(get_block_ids(input_test), correct_output);
    }
    #[test] 
    fn bigstring_to_number() {
        let input_test = "00...111...2...333.44.5555....66.777.888899".chars().map(|x| {x.to_string()}).collect::<Vec<String>>();
        assert_eq!(refragment(input_test), 1485);
    }
    #[test]
    fn whole_thing() {
        let input_string = fs::read_to_string("inputs/test").unwrap();
        let ids = get_block_ids(input_string);
        assert_eq!(1928, refragment(ids));
    }
}