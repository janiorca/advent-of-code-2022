use std::{fs, collections::HashMap};

fn build_map( shape_lines: &Vec<Vec<(i64,i64)>> ) -> HashMap<(i64,i64),u8> {
    let mut map: HashMap<(i64,i64),u8> = HashMap::new();

    for line in shape_lines {
        let mut points = line.iter();   
        let mut cursor = points.next().unwrap().clone();
        map.insert((cursor.0, cursor.1),1);
        while let Some(next_point) = points.next() {
            let step = ((next_point.0-cursor.0).signum(),(next_point.1-cursor.1).signum());
            while cursor != *next_point {
                cursor = (cursor.0 + step.0, cursor.1+step.1);
                map.insert((cursor.0, cursor.1),1);
            }
        } 
    }
    return map;
}

fn simulate( mut map :HashMap<(i64,i64),u8>, with_extra_floor: bool ) -> u64 {
    let max_y = map.clone().into_keys().map(|x|x.1).max().unwrap();
    // simulate
    let mut sand_drops = 0;
    'outer: loop {
        let mut drop_pos = (500i64, 0i64 );
        loop {
            if with_extra_floor {
                if drop_pos.1 == max_y+1 {
                    map.insert(drop_pos,2);
                    sand_drops += 1;
                    break;
                }
            } else {
                // Check if outside the world
                if drop_pos.1 > max_y {
                    break 'outer;
                }
            }
            if !map.contains_key( &(drop_pos.0, drop_pos.1+1) ) {  
                // go straight down
                drop_pos.1 += 1;
            } else if !map.contains_key( &(drop_pos.0-1, drop_pos.1+1) ) {  
                // down and left
                drop_pos.1 += 1;
                drop_pos.0 -= 1;
            } else if !map.contains_key( &(drop_pos.0+1, drop_pos.1+1) ) {  
                // dwn and right
                drop_pos.1 += 1;
                drop_pos.0 += 1;
            } else {
                // still here. We could not move the sand piece. Which means we could have finished task 2. 
                if with_extra_floor && drop_pos == (500i64, 0i64 ) {
                    return sand_drops+1;
                }
                // othrwise we place the sand particle
                map.insert(drop_pos,2); 
                sand_drops += 1;
                break;
            }
        }
    }
    return sand_drops;

}

fn main() {
    // Part1 & Part2
    let input = fs::read_to_string("inputs/aoc14.txt").unwrap();
    let lines = input.lines();
    let mut shape_lines: Vec<Vec<(i64,i64)>> = Vec::new();
    for line in lines{
        let parts: Vec<&str> = line.split_whitespace().filter(|x| *x!= "->" ).collect();
        let mut coord_list: Vec<(i64,i64)> = Vec::new();
        for part in &parts {
            let coords: Vec<&str> = part.split(',').collect(); 
            let (x, y) = (coords[0].parse::<i64>().unwrap(), coords[1].parse::<i64>().unwrap() );
            coord_list.push((x,y));
        }
        shape_lines.push(coord_list);
    }

    let map = build_map(&shape_lines);
    println!( "Sand drops {}", simulate(map, false));   // Part 1

    let map = build_map(&shape_lines);
    println!( "Sand drops with extra floor{}", simulate(map, true)); // Part 2
}
