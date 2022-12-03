use std::{fs, collections::{BinaryHeap, HashSet}, hash::Hash};

fn score_char( c: char ) -> u8 {
    if c >= 'a' && c <= 'z' {
        return c as u8 -  ( 'a' as u8 ) + 1;
    } else {
        return  c as u8 -  ( 'A' as u8 ) + 27;
    }
}

fn main() {
    // Part1 
    let input = fs::read_to_string("inputs/aoc3.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>(); 
    let mut total_priority: u32 = 0;
    for line in &lines {
        let half_len = line.len()/2;
        let left_set = HashSet::<char>::from_iter( line[0..half_len].to_owned().chars());
        let right_set = HashSet::<char>::from_iter( line[half_len..].to_owned().chars());

        let shared_set: Vec::<&char> = left_set.intersection(&right_set).collect();
        let c = *shared_set[0];
        let score = score_char(c);
        total_priority += score as u32;
    }
    println!( "priority sum {}", total_priority);

    // Part 2
    let mut total_priority: u32 = 0;
    for n in (0..lines.len()).step_by(3) {
        let ss1: HashSet<char> = HashSet::<char>::from_iter( lines[n].chars() ); 
        let ss2: HashSet<char> = HashSet::<char>::from_iter( lines[n+1].chars() ); 
        let ss3: HashSet<char> = HashSet::<char>::from_iter( lines[n+2].chars() ); 
        let merged = ss1.intersection(&ss2).cloned().collect::<HashSet<char>>();
        let merged = merged.intersection(&ss3).cloned().collect::<Vec<char>>();
        let c = *merged.get(0).unwrap();
        total_priority += score_char(c) as u32;
    }
    println!( "priority sum {}", total_priority);

}
