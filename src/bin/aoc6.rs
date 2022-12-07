use std::{fs, collections::{HashSet, HashMap}};

fn is_unique( input: &[u8] ) -> bool {
    let mut chars: HashSet<u8> = HashSet::new();
    for byte in input {
        chars.insert(*byte);
    }
    return chars.len() == input.len();
}

fn main() {
    let input = fs::read_to_string("inputs/aoc6.txt").unwrap().into_bytes();

    for index in 0..input.len()-4 {
        if is_unique(&input[index..index+4]) {
            println!( "Marker at {}", index+4);
            break;
        }
    }

    // Part 2
    for index in 0..input.len()-14 {
        if is_unique(&input[index..index+14]) {
            println!( "Message Marker at {}", index+14);
            break;
        }
    }
}
