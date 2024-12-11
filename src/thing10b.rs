use std::fs;

pub fn main() {
    let input_string = fs::read_to_string("inputs/day10").unwrap();
    let grid = input_string.lines()
        .map(|line| line.chars()
            .map(|c| c.to_string().parse::<usize>().unwrap_or(10)).collect::<Vec<_>>()
        ).collect::<Vec<_>>();
    let mut search = SearchState{trailheads: vec!(), places_to_search: vec!()};
    find_trailheads(&grid, &mut search);
    println!("{:?}", search);
    while search.places_to_search.len() > 0 {
        next_from_queue(&grid, &mut search);
    }
    println!("{:?}", search);
    println!("{}", search.sum());
}

fn find_trailheads(grid: &Vec<Vec<usize>>, search: &mut SearchState) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 0 {
                let entry = QueueEntry {
                    x, 
                    y, 
                    id: search.trailheads.len()
                };
                search.places_to_search.push(entry);
                search.trailheads.push(vec![]);
            }
        }
    }
}

fn next_from_queue(grid: &Vec<Vec<usize>>, search: &mut SearchState) {
    let current = search.places_to_search.pop().unwrap();
    let next_number = grid[current.y][current.x] + 1;

    let deltas = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    for delta in deltas {
        let new_x = (current.x as isize + delta.0) as usize;
        let new_y = (current.y as isize + delta.1) as usize;

        if new_x >= grid[0].len() || new_y >= grid.len() {
            continue;
        }

        if grid[new_y][new_x] == next_number {
            if next_number == 9 {
                //if search.trailheads[current.id].contains(&(new_x, new_y)) {
                //    continue;
                //}
                println!("{new_x},{new_y}");
                search.trailheads[current.id].push((new_x, new_y));
                continue;
            }
            let new_entry = QueueEntry {
                x: new_x,
                y: new_y,
                id: current.id,
            };
            search.places_to_search.push(new_entry);
        }
    }
}

#[derive(Debug)]
struct SearchState {
    trailheads: Vec<Vec<(usize, usize)>>,
    places_to_search: Vec<QueueEntry>,
}

impl SearchState {
    pub fn sum(&self) -> usize {
        self.trailheads.clone().iter().map(|x| x.len()).sum()
    }
}

#[derive(Debug)]
struct QueueEntry {
    x: usize,
    y: usize,
    id: usize,
}