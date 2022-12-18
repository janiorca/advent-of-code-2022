use std::{fs, collections::{HashSet, VecDeque}};

fn main() {
    // Part1
    let input = fs::read_to_string("inputs/aoc18.txt").unwrap();
    let mut cube_map: HashSet<(i64,i64,i64)> = HashSet::new();
    for line in input.lines() {
        let parts = line.split(',').collect::<Vec<&str>>();
        let x = parts[0].parse::<i64>().unwrap();
        let y = parts[1].parse::<i64>().unwrap();
        let z = parts[2].parse::<i64>().unwrap();
        cube_map.insert((x,y,z));
    }

    // Part 1
    let mut surface_area = 0;
    let directions = [(0,0,1),(0,0,-1),(0,1,0),(0,-1,0),(1,0,0),(-1,0,0)];
    for cube in &cube_map {
        let mut cube_area = 6;
        for direction in &directions {
            let test_pos = (cube.0 + direction.0, cube.1 + direction.1, cube.2 + direction.2 );
            if cube_map.contains(&test_pos) {
                cube_area -= 1;
            }            
        }
        surface_area += cube_area;
    }
    println!("surface_area: {}", surface_area);

    // Part 2
    // Find range to constrain next stage
    let mut min = (100,100,100); 
    let mut max = (0,0,0); 
    for cube in &cube_map {
        min.0 = min.0.min( cube.0 ); 
        min.1 = min.1.min( cube.1 ); 
        min.2 = min.2.min( cube.2 ); 
        max.0 = max.0.max( cube.0 ); 
        max.1 = max.1.max( cube.1 ); 
        max.2 = max.2.max( cube.2 ); 
    }

    // Flood fill the area outside the rock ( constrain into a cube slighly larger than the contained rock to ensure the fill can reach all outside areas)
    min = (min.0 - 1, min.1 -1, min.2 -1 ); 
    max = (max.0 + 1, max.1 +1, max.2 +1 ); 
    let mut water_map: HashSet<(i64,i64,i64)> = HashSet::new();
    let mut fill_queue: VecDeque<(i64,i64,i64)> = VecDeque::from( [(0,0,0)]);
    while let Some( fill_pos ) = fill_queue.pop_front() {
        if water_map.contains(&fill_pos) {
            continue;
        }
        water_map.insert(fill_pos);
        for direction in &directions {
            let test_pos = (fill_pos.0 + direction.0, fill_pos.1 + direction.1, fill_pos.2 + direction.2 );
            if test_pos.0 < min.0 || test_pos.1 < min.1 || test_pos.2 < min.2 || test_pos.0 > max.0 || test_pos.1 > max.1 || test_pos.2 > max.2 {
                continue;
            }
            // IS there a cube blocing the water
            if !cube_map.contains(&test_pos) {
                // Is there already water
                if !water_map.contains(&test_pos) {
                    fill_queue.push_back(test_pos);                    
                }
            }
        }
    }

    // Perform the same areas calc as in part 1 inverted so that a nbor is added to the water surace area if it is water
    let mut water_surface_area = 0;
    let directions = [(0,0,1),(0,0,-1),(0,1,0),(0,-1,0),(1,0,0),(-1,0,0)];
    for cube in &cube_map {
        let mut cube_area = 0;
        for direction in &directions {
            let test_pos = (cube.0 + direction.0, cube.1 + direction.1, cube.2 + direction.2 );
            if water_map.contains(&test_pos) {
                cube_area += 1;
            }            
        }
        water_surface_area += cube_area;
    }
    println!("water_surface_area: {}", water_surface_area);

}
