use std::{fs, collections::HashMap};

fn main() {
    let input = fs::read_to_string("inputs/aoc7.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>(); 

    let mut path: Vec<String> = Vec::new();
    let mut dir_size: HashMap<String,u64> = HashMap::new();
    let mut line_index = 0;
    while line_index < lines.len() {
        let command = lines[ line_index ];
        let parts = command.split(' ').collect::<Vec<&str>>();
        if parts[0] != "$" {
            panic!( "Unexpected character")
        }
        line_index += 1;
        match parts[1] {
            "cd" => {
                if parts[2] == "/" {
                    path.clear();
                } else if parts[2] == ".." {
                    path.pop();
                }else {
                    path.push( parts[2].to_string() );
                }
            }
            "ls" => {
                let mut files_size = 0; 
                while line_index < lines.len() {
                    let line = lines[ line_index];
                    let line_parts = line.split(' ').collect::<Vec<&str>>();
                    if line_parts[0] == "$" {
                        break;
                    }
                    line_index += 1;
                    if line_parts[0] != "dir" {
                        let size = line_parts[0].parse::<u64>().unwrap();
                        files_size += size;
                    }
                }
                let path_key = path.iter().cloned().fold("/".to_string(), |sub_path, new_part| sub_path.to_owned() + "/" + &new_part );
                println!( "path: {},  {}", path_key, files_size);
                dir_size.insert(path_key, files_size);
            }
            _ => panic!("unexpected commans")
        }
    }

    let mut sum_of_totals = 0;
    let mut size_by_path = HashMap::<String,u64>::new(); 
    for sub_dir in dir_size.keys() {
        let mut total_size = 0;
        for entry in &dir_size {
            if entry.0.contains(sub_dir) {
                total_size += entry.1;
            }
        }
        size_by_path.insert(  sub_dir.clone(), total_size );
        if total_size <= 100000 {
            sum_of_totals += total_size;
        }
    }
    // Part 1
    println!( "sum of totals; {}", sum_of_totals);

    let total_used = size_by_path[ "/" ];
    let free = 70000000 - total_used;
    let required_min = 30000000 - free; 
    let mut best_size = 99999999;
    // PArt 2
    for sub_dir in dir_size.keys() {
        let path_size = size_by_path[ sub_dir ];
        if path_size > required_min && path_size < best_size {
            best_size = path_size;
        } 
    }
    // Part2
    println!( "best size; {}", best_size);

}
