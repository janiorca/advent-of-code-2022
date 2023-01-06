use std::{collections::HashSet, fs};

fn modulo( x: i32, y: i32 ) -> i32 {
    return ((x % y)+ y)%y; 
}

fn can_pass( pos: (i32,i32), blizzard_map: &Vec<Vec<char>>, time: i32 ) -> bool {
    let map_height = blizzard_map.len() as i32;
    let map_width = blizzard_map[0].len() as i32;

    // stop falling off the map
    if pos.1 < 0 || pos.1 == map_height{
        return false;
    }

    if blizzard_map[pos.1 as usize][pos.0 as usize] == '#' {
        return false;
    }

    // stop and start positions are always OK
    if pos.1 == 0 || pos.1 == map_height-1 {
        return true;
    }
    // Check each direction
    for (ch, dir) in [('v', (0,-1)), ('^', (0,1)), ('>', (-1,0)), ('<', (1,0))] {
        let sideless_x = modulo((pos.0-1) + dir.0*time, map_width-2)+1;
        let sideless_y = modulo((pos.1-1) + dir.1*time, map_height-2)+1;
        if blizzard_map[sideless_y as usize][sideless_x as usize] == ch {
            return false;
        }
    } 
    return true;
}

fn find_shortest_path( blizzard_map: &Vec<Vec<char>>, start_pos: (i32,i32), end_pos: (i32,i32), mut step_no: i32 ) -> i32 {
    let mut open_positions: HashSet<(i32,i32)> = HashSet::new();
    open_positions.insert(start_pos); 
    loop {
        step_no += 1;
        let mut next_positions: HashSet<(i32,i32)> = HashSet::new();
        for pos in open_positions {
            if can_pass(pos, &blizzard_map, step_no) {
                next_positions.insert(pos);
            }
            for step in [ (0,1), (0,-1),(1,0), (-1,0)] {
                let potential_pos = (pos.0 + step.0, pos.1 + step.1 );
                if potential_pos == end_pos {
                    return step_no;
                }
                if can_pass(potential_pos, &blizzard_map, step_no) {
                    next_positions.insert(potential_pos);
                }
            }
        }
        open_positions = next_positions;
    }
}

fn main() {
    let input = fs::read_to_string("inputs/aoc24.txt").unwrap();
    let mut blizzard_map: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        blizzard_map.push( line.chars().collect::<Vec<char>>() );
    }
    let start_pos = (1,0);
    let end_pos = (blizzard_map[0].len() as i32 - 2, blizzard_map.len() as i32 -1 );

    // part 1
    let turns = find_shortest_path( &blizzard_map, start_pos,  end_pos, 0 );
    println!( "{}: turns to reach exit", turns);

    // part 2
    let mut trip_turns = find_shortest_path( &blizzard_map, start_pos,  end_pos, 0 );
    trip_turns = find_shortest_path( &blizzard_map, end_pos,  start_pos, trip_turns );
    trip_turns = find_shortest_path( &blizzard_map, start_pos,  end_pos, trip_turns );
    println!( "{}: turns to reach exit", trip_turns);
}
