use std::fs;
#[allow(dead_code)]

pub fn main() {
    let input_string = fs::read_to_string("inputs/day5").unwrap();
    let mut rules: Vec<(usize,usize)> = vec!();
    let mut orders: Vec<Vec<usize>> = vec!();
    for line in input_string.lines(){
        if line.contains("|") {
            let numbers: Vec<usize> = line.split("|").map(|x| x.parse::<usize>().unwrap()).collect();
            rules.push((numbers[0], numbers[1]));
        } else if line.contains(",") {
            let numbers: Vec<usize> = line.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
            orders.push(numbers);
        }
    }

    let mut total = 0;

    for order in orders {
        let result = input_against_rules(order, rules.clone());
        println!("{:?}", result);
        if let Some(outcome) = result {
            println!("{}", outcome[outcome.len()/2]);
            total += outcome[outcome.len()/2];
        }
    }
    println!("{}", total);
}

fn input_against_rules(order: Vec<usize>, rules: Vec<(usize, usize)>) -> Option<Vec<usize>> {
    for bogo in order.clone() {
        let relevant_rules = rules.clone().into_iter().filter(|(a, b)| {
            *a == bogo || *b == bogo
        }).collect::<Vec<(usize, usize)>>();
        for (must_first, must_later) in relevant_rules {
            let index_first;
            let index_second;

            if let Some(index) = order.clone().into_iter().position(|x| x == must_first){
                index_first = index;
            } else {
                //println!("couldnt find {} in {:?}", must_first, order);
                continue;
            }
            
            if let Some(index) = order.clone().into_iter().position(|x| x == must_later){
                index_second = index;
            } else {
                //println!("couldnt find {} in {:?}", must_later, order);
                continue;
            }
            if index_first > index_second {
                //println!("indicies {}, {} wrong way round", must_first, must_later);
                return None;
            }

        }
    }
    return Some(order);
}