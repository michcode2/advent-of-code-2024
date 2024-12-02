use std::fs;

pub fn main(){
    let thing = fs::read_to_string("inputs/day2").unwrap();
    let lines = thing.lines();
    let mut dangerous = 0;
    for line in lines.clone() {
        let numbers_str = line.split(" ");
        let numbers = numbers_str.map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let increasing = numbers[0] < numbers[1];
        for i in 1..numbers.len(){
            let difference = usize::abs_diff(numbers[i], numbers[i-1]);
            if ((difference > 3) || (difference < 1)) {
                dangerous += 1;
                break;
            }
            if ((numbers[i-1] < numbers[i]) ^ increasing) {
                dangerous += 1;
                break;
            }
        }
    }
    println!("{}", lines.count() - dangerous);
}