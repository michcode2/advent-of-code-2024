use std::fs;

pub fn main() {
    let input = fs::read_to_string("inputs/day3").unwrap();
    let instructions = input.split("m");
    let mut total = 0;
    let mut enabled = true;
    for input in instructions {
        let (result, return_enabled) = validate_instruction(input.to_owned(), enabled);

        enabled = return_enabled;
        if let Some(yippee) = result{
            total += yippee;
        }
    }
    println!("{}",total);
}

fn validate_instruction(possible_instruction: String, enabled: bool) -> (Option<usize>, bool) {
    let mut number_1 = 1234;
    let mut index_1 = 0; 
    
    let mut return_enabled = enabled;

    if possible_instruction.contains("do()") {
        return_enabled = true;
        print!("enable ");
    } else if possible_instruction.contains("don't()") {
        return_enabled = false;
        print!("disable ");
    }

    let end_of_string = std::cmp::min(7, possible_instruction.len());

    for i in 3..end_of_string {
        let sliced_string = &possible_instruction[3..i];
        if let Ok(sliced_number) = sliced_string.parse::<usize>(){
            number_1 = sliced_number;
            index_1 = i;
        }
        else {
            continue;
        }
    }
    
    let mut number_2 = 1234;
    let mut index_2 = 0;

    let end_of_string = std::cmp::min(index_1+5, possible_instruction.len());

    for i in index_1 + 1..end_of_string {
        let sliced_string = &possible_instruction[index_1+1..i];
        if let Ok(sliced_number) = sliced_string.parse::<usize>(){
            number_2 = sliced_number;
            index_2 = i;
        }
        else {
            continue;
        }
    }

    println!("{}, {}, {}, {}, {}", possible_instruction, number_1, number_2, enabled, return_enabled);

    if number_2 > 999 {
        return (None, return_enabled);
    }
    if number_1 > 999 {
        return (None, return_enabled);
    }
    if !(&possible_instruction[..3] == "ul(") {
        return (None, return_enabled);
    }
    if !possible_instruction.contains(")") {
        return (None, return_enabled);
    }
    if !(&possible_instruction[index_2..index_2 + 1] == ")") {
        return (None, return_enabled);
    }
    
    let value_to_return;
    if enabled {
        value_to_return = number_1 * number_2;
    }
    else {
        value_to_return = 0;
    }
    
    return (Some(value_to_return), return_enabled);
}