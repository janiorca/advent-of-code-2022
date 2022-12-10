use std::{fs, collections::HashSet};

fn simulate_rope( num_knots: usize) -> usize {
    let input = fs::read_to_string("inputs/aoc9.txt").unwrap();
    let mut rope_positions = vec![(0,0);num_knots];
    let mut tail_positions = HashSet::<(i32,i32)>::new();
    for line in input.lines() {
        let command_parts: Vec<&str> = line.split(' ').collect();
        let num_moves = command_parts[1].parse::<u32>().unwrap();

        for _move_no in 0..num_moves {
            // Update head
            match command_parts[0] {
                "U" => rope_positions[0].1 += 1,
                "D" => rope_positions[0].1 -= 1,
                "L" => rope_positions[0].0 -= 1,
                "R" => rope_positions[0].0 += 1,
                _ => { panic!( "Unrecognized command"); }
            }
            // update other positions
            for rope_index in 1..num_knots {
                let delta: (i32,i32) = (rope_positions[rope_index-1].0-rope_positions[rope_index].0, rope_positions[rope_index-1].1-rope_positions[rope_index].1);
                if delta.0.abs() <=1 && delta.1.abs() <=1 {
                    continue;
                }
                rope_positions[rope_index] = ( rope_positions[rope_index].0 + delta.0.signum(), rope_positions[rope_index].1 + delta.1.signum() );
            }
            tail_positions.insert(rope_positions[num_knots-1]);
        }
    }
    return tail_positions.len();
}

fn main() {
    // Part1
    println!( "unique tail positions: {} ", simulate_rope(2));

    // Part 2
    println!( "unique long tail positions: {} ", simulate_rope(10));
}