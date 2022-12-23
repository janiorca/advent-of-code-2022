use std::fs;

#[derive(Clone,PartialEq, Copy)]
enum Terrain{
    Void,
    Passable, 
    Blocked
}

fn calc_new_pos( pos: &(i32,i32), direction: usize, puzzle_size: (i32,i32)) -> (i32,i32) {
    let directions = [ (1,0), (0,1), (-1,0), (0,-1)];

    let mut new_pos = (pos.0 + directions[ direction ].0, pos.1 + directions[ direction ].1 );
    new_pos = ((new_pos.0 + puzzle_size.0)%puzzle_size.0, (new_pos.1 + puzzle_size.1)%puzzle_size.1 );
    return new_pos
}

fn main() {
    let input = fs::read_to_string("inputs/aoc22.txt").unwrap();

    let mut puzzle_map: Vec<Vec<Terrain>> = Vec::new();
    let mut lines_iter = input.lines();
    loop {
        let line = lines_iter.next().unwrap();
        if line == "" {
            break;
        }
        let puzzle_line = line.chars().map(|x| match x {
                    ' ' => Terrain::Void,
                    '.' => Terrain::Passable,
                    '#' => Terrain::Blocked,
                    _ => panic!( "unexpected terrain")
                }).collect::<Vec<Terrain>>();
        puzzle_map.push( puzzle_line );
    }
    let instructions = lines_iter.next().unwrap();
    let puzzle_width = puzzle_map.iter().map(|x|x.len()).max().unwrap() as i32;
    let puzzle_height = puzzle_map.len() as i32;
    // Pad out all puzzle lines to be the same width to make the movement calculations easier. They are ony shorter if voids at end of lines were omitted
    for puzzle_line in &mut puzzle_map {
        puzzle_line.extend(vec![Terrain::Void;(puzzle_width-puzzle_line.len() as i32) as usize]);
    }

    // Find start x
    let mut pos = (puzzle_map[0].iter().position(|x|*x==Terrain::Passable).unwrap() as i32, 0 as i32 );
    let mut direction = 0;
    let instruction_string = instructions.to_owned().replace("L", " L ").replace("R", " R ");
    let inst_parts = instruction_string.split_ascii_whitespace().collect::<Vec<&str>>();
    for instruction in inst_parts {
        let num_steps = instruction.parse::<i64>();
        if num_steps.is_ok() {
            let mut steps_remaining = num_steps.unwrap();
            let mut new_pos = pos;
            while steps_remaining > 0 {
                new_pos = calc_new_pos( &new_pos, direction, (puzzle_width, puzzle_height ));
                let terrrain_piece = puzzle_map[ new_pos.1 as usize][ new_pos.0  as usize];
                match terrrain_piece {
                    Terrain::Blocked => break,
                    Terrain::Passable => {
                        pos = new_pos;
                        steps_remaining -= 1;
                    },
                    Terrain::Void => { }
                }
            }
        } else {
            direction = if instruction == "L" { (direction+3)%4 } else { (direction+1)%4};
        }
    }
    let password = 1000*(pos.1+1)+4*(pos.0+1)+direction as i32;

    println!( "password {}", password );
}