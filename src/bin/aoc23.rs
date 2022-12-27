use std::{collections::{HashSet, HashMap}, fs};

fn get_input() -> HashSet<(i32,i32)>{
    let mut elves: HashSet<(i32,i32)> = HashSet::new();
    let input = fs::read_to_string("inputs/aoc23.txt").unwrap();
    for (y, row) in input.lines().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if ch == '#' {
                elves.insert((x as i32, y as i32));
            }
        }
    }
    return elves
}

fn run_simulation( mut elves: HashSet<(i32,i32)>, max_rounds: i32 ) -> ( i32, i32 ) {
    let directions = [  
        // main direction + two probes
        (0,-1), (-1,-1), (1,-1),
        (0,1), (-1,1), (1,1),
        (-1,0), (-1,-1), (-1,1),
        (1,0), (1,-1), (1,1) ];

    let mut current_main_dir = 0;
    let mut round = 1;
    while round <  max_rounds {
        let mut proposed_moves: Vec<((i32,i32),(i32,i32))> = Vec::new();
        for elf_pos in &elves {
            let mut is_clear = true;
            for nbor_dir in directions {
                let check_pos = (elf_pos.0+ nbor_dir.0, elf_pos.1+ nbor_dir.1 );
                if elves.contains(&check_pos) {
                    is_clear = false;
                    break;
                }
            }
            if is_clear { 
                continue;
            }
            'outer: for try_step in 0..4 {
                let dir_index = ((current_main_dir + try_step)%4)*3;
                for probe in 0..3 {
                    let probe_pos = (elf_pos.0 + directions[dir_index+probe].0, elf_pos.1 + directions[dir_index+probe].1 );
                    if elves.contains(&probe_pos) {
                        continue 'outer;
                    }
                }
                // Still here, a candidate move
                let proposed_elf_pos = (elf_pos.0 + directions[dir_index].0, elf_pos.1 + directions[dir_index].1 );
                proposed_moves.push( (elf_pos.clone(), proposed_elf_pos) );
                break;
            }
        }
        if proposed_moves.len() == 0 {
            break;            
        }
        // Identify any destinations that are present at least twice
        let destinations: Vec<(i32,i32)> = proposed_moves.iter().cloned().map(|x|x.1).collect::<Vec<(i32,i32)>>();
        let mut destination_counts: HashMap<(i32,i32),i32> = HashMap::new();
        for dest in destinations {
            *destination_counts.entry( dest ).or_insert(0) += 1; 
        }
        
        // filter proposed move to only include those that dont have doubled destinatiopns
        let final_moves: Vec<((i32,i32),(i32,i32))> = proposed_moves.iter().cloned().filter(|x| destination_counts[ &x.1 ] ==  1 ).collect();
        for elf_move in final_moves {
            elves.remove(&elf_move.0);
            elves.insert(elf_move.1);
        }
        current_main_dir += 1;
        round += 1;
    }

    let mut min_pos = (100,100);
    let mut max_pos= (-100,-100);
    for elf_pos in &elves {
        min_pos.0 = min_pos.0.min( elf_pos.0);
        min_pos.1 = min_pos.1.min( elf_pos.1);
        max_pos.0 = max_pos.0.max( elf_pos.0);
        max_pos.1 = max_pos.1.max( elf_pos.1);
    }
    let elf_area = (1 + max_pos.0 - min_pos.0 ) * (1 + max_pos.1 - min_pos.1 ) - elves.len() as i32;

    return ( elf_area, round );
}

fn main() {
    // part 1
    let elves = get_input();
    let (elf_area, _rounds ) = run_simulation(elves, 10);
    println!( "elf area: {}", elf_area);

    let elves = get_input();
    let (_elf_area, rounds ) = run_simulation(elves, 100000);
    println!( "required round: {}", rounds);

}
