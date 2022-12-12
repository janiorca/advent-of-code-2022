use std::{fs, collections::{VecDeque, HashMap}};

fn find_path_length( start: (i32,i32), end: (i32,i32), height_map: &Vec<Vec<i32>> ) -> i32 {
    let width = height_map[0].len() as i32;
    let height = height_map.len() as i32;
    let mut open_searches: VecDeque<((i32,i32),u32)> = VecDeque::new();
    open_searches.push_back( (start,1));
    let mut visited_cells: HashMap<(i32,i32),u32> = HashMap::new();

    while let Some( (current_cell,current_cost)) = open_searches.pop_front() {
        if visited_cells.contains_key(&current_cell) {
            continue;
        }
        visited_cells.insert(current_cell, current_cost);
        for dir in [(1,0), (-1,0), (0,1),(0,-1)] {
            let potential = (current_cell.0 + dir.0, current_cell.1+dir.1);
            if potential.0 < 0 || potential.1 < 0 || potential.0 >= width || potential.1 >= height {
                continue;
            }
            if height_map[potential.1 as usize ][potential.0 as usize]  - height_map[current_cell.1 as usize][current_cell.0 as usize] > 1 {
                continue;
            }
            if potential == end {
                return *visited_cells.get(&current_cell).unwrap() as i32;
            }
            open_searches.push_back((potential, current_cost+1));
        }
    }
    return 99999;
}

fn main() {
    let input = fs::read_to_string("inputs/aoc12.txt").unwrap();
    let mut starting_positions: Vec<(i32,i32)> = Vec::new();        // need these for part2
    let mut height_map: Vec<Vec<i32>> = Vec::new();
    let mut start = (0i32,0i32); 
    let mut end = (0i32,0i32); 
    let mut y=0;
    for line in input.lines() {
        height_map.push( Vec::new() );
        let mut x=0;
        for c in line.chars() {
            match c {
                'S' => {
                    start = (x,y);
                    height_map[y as usize].push(0);
                }
                'E' => {
                    end = (x,y);
                    height_map[y as usize].push('z' as i32 -'a' as i32 ) ;
                }
                _ => {
                    height_map[y as usize].push(c as i32 -'a' as i32 ) ;
                }
            }
            if height_map[y as usize][x as usize] == 0 {
                starting_positions.push((x,y));
            }
            x += 1;
        }
        y += 1;
    }
    println!( "Start:{:?}    End:{:?}", start,end);

    // Part 1
    println!( "Path length from S {}", find_path_length(start, end, &height_map));

    // Part 2
    let mut shortest_length = 99999;
    for start in starting_positions {
        let length = find_path_length(start, end, &height_map);
        if length < shortest_length {
            shortest_length = length;
        }
    }
    println!( "Shortest path length from an a {}", shortest_length)
 }
