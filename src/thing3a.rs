use std::fs;

pub fn main() {
    let input = fs::read_to_string("inputs/day3").unwrap();
    let instructions = input.split("m");
    let mut total = 0;
    for input in instructions {
        if let Some(yippee) = validate_instruction(input.to_owned()) {
            total += yippee;
        }
    }
    println!("{}",total);
}

fn validate_instruction(possible_instruction: String) -> Option<usize> {
    let mut number_1 = 1234;
    let mut index_1 = 0; 

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

    if number_2 > 999 {
        return None;
    }
    if number_1 > 999 {
        return None;
    }
    if !(&possible_instruction[..3] == "ul(") {
        return None;
    }
    if !possible_instruction.contains(")") {
        return None;
    }
    if !(&possible_instruction[index_2..index_2 + 1] == ")") {
        return None;
    }
    println!("{}, {}, {}", possible_instruction, number_1, number_2);
    return Some(number_1 * number_2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn handle_individual_inputs() {
        assert_eq!(validate_instruction("ul(44,46)".to_owned()), Some(44*46));
        assert_eq!(validate_instruction("ul(123,4)".to_owned()), Some(123*4));
        assert_eq!(validate_instruction("ul(4*".to_owned()), None);
        assert_eq!(validate_instruction("ul(6,9!".to_owned()), None);
        assert_eq!(validate_instruction("ul( 2 , 4 )".to_owned()), None);
    }

    #[test]
    fn more_complex_test() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_owned(); 
        let instructions = input.split("m");
        let mut total = 0;
        for input in instructions {
            if let Some(yippee) = validate_instruction(input.to_owned()) {
                total += yippee;
            }
        }
    }
}