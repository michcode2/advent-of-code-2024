use std::{fmt::Debug, fs, os::unix::fs::PermissionsExt};
#[allow(dead_code)]

pub fn main() {
    let input_string = fs::read_to_string("inputs/day9").unwrap();
    let splitted = split_into_chunks(input_string);
    let moved = move_to_first_space(splitted);
    calculate_checksum(moved);
}

fn split_into_chunks(input: String) -> Vec<FileSpace> {
    let mut chunks = vec![];
    let mut file = true;
    let mut current_id = 0;
    for num in input.chars() {
        let num_as_int = num.to_string().parse::<usize>().unwrap();
        if file {
            chunks.push(FileSpace{
                is_file: true,
                size: num_as_int,
                id: Some(current_id),
                include_in_search: true,
            });
            current_id += 1;
        } else {
            if num_as_int > 0 {
                chunks.push(FileSpace{
                    is_file: false,
                    size: num_as_int,
                    id: None,
                    include_in_search: false,
                });
            }
        }
        file = !file;
    }

    println!("split into chunks");
    //println!("{:?}\n", chunks);
    return chunks;
}

fn move_to_first_space(mut chunks: Vec<FileSpace>) -> Vec<FileSpace> {
    loop {
        let mut this_file = FileSpace{
            is_file: false,
            size: 0,
            id: None,
            include_in_search: false,
        };

        let mut reverse_index = 0;
        loop {
            // finds the next numbr that is free to move
            reverse_index += 1;
            if reverse_index == chunks.len(){
                return chunks;
            }
            let index_to_change = chunks.len() - reverse_index;
            let value = chunks[index_to_change];
            //if !value.is_file {
              //  reverse_index = 0;
                //chunks.remove(index_to_change);
                //println!("got rid of the last thing");
            //}
            this_file = value;
            
            if this_file.include_in_search{
                break;
            }
        }

        if this_file.size == 0 {
            break;
        }

        let index_to_change = chunks.len() - reverse_index;
        println!("{}", index_to_change);
        chunks[index_to_change].include_in_search = false;

        // for i in 0..change.len() {  ----- THIS IS INCREDIBLY SLOW HOLY SHIT
        for i in 0..index_to_change {
            if chunks[i].is_file {
                // dont replace a file with a file
                continue;
            }
            let mut to_remove = index_to_change;
            if chunks[i].size >= this_file.size {
                if chunks[i].size > this_file.size {
                    // make sure any extra space stays free
                    let new_space = FileSpace{
                        is_file: false,
                        size: chunks[i].size - this_file.size,
                        id: None,
                        include_in_search: false,
                    };
                    //chunks = insert(chunks.clone(), i+1, new_space);
                    chunks.insert(i+1, new_space);
                    //chunks.insert(i+1, new_space);
                    //assert_eq!(chunks, test);
                    to_remove += 1;
                }
                // put it in the right place
                if i < to_remove {
                    chunks[i] = this_file.clone();
                    chunks[i].include_in_search = false;
                    chunks[to_remove] = FileSpace {
                        is_file: false,
                        size: this_file.size,
                        id: None,
                        include_in_search: false,
                    };
                    break;
                }
            }
        }
        
        //if let None = chunks.clone().iter().position(|x| x.include_in_search == false) {
        //    break;
        //}
    }
    println!("{:?}", chunks);
    return chunks;
}

fn calculate_checksum(input: Vec<FileSpace>) {
    println!("{:?}", input);
    let mut place = 0;
    let mut total = 0;
    for space in input {
        for i in 0..space.size {
            if let Some(value) = space.id {
                total += value * place;
            }
            place += 1;
        }
    }
    println!("{total}");
}

#[derive(Clone, Copy, PartialEq)]
struct FileSpace {
    is_file: bool,
    size: usize,
    id: Option<usize>,
    include_in_search: bool,
}

impl Debug for FileSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut thingy = "".to_string();
        let representer: String;
        if self.is_file {
            representer = self.id.unwrap().to_string();
        } else {
            representer = "-".to_string();
        }
        for _ in 0..self.size {
            thingy.push_str(&representer);
        }
        f.write_str(&thingy)
    }
}