use std::fs;

pub fn main() {
    let thing = fs::read_to_string("inputs/day1").unwrap();

    let (mut list_l, mut list_r): 
        (Vec<usize>, Vec<usize>) = 
        thing.lines().map(|line| {
            let strings: Vec<&str> = line.split(" ").collect();
        
            (strings[0].parse::<usize>().unwrap(), 
                strings.last().unwrap().parse::<usize>().unwrap())
    }).collect();

    list_l.sort();
    list_r.sort();

    let mut score = 0;
    for to_test in list_l {
        let times_in_r = list_r.clone().into_iter().filter(|&x| x == to_test).collect::<Vec<usize>>().len();
        score += times_in_r * to_test;
    }

    println!("{}", score);
}
