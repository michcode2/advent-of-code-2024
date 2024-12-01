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

    let mut both = list_l.into_iter().zip(list_r.into_iter());
    
    let total: usize = both.map(|(l, r)| {
        usize::abs_diff(r, l)
    }).sum();

    println!("{}", total);
}
