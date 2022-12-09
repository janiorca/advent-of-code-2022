use std::{fs, collections::{HashSet, HashMap}, hash::Hash};

fn update_scan( x: usize, y: usize, highest_seen: &mut i8, tree_mask: &mut Vec<Vec<u32>>, tree_map: &Vec<Vec<u8>>) {
    if tree_map[y][x] as i8 > *highest_seen {
        tree_mask[y][x] = 1;
        *highest_seen = tree_map[y][x] as i8;
    }
}

fn cast_ray( mut pos: (i32,i32), dir: (i32, i32), tree_map: &Vec<Vec<u8>> ) -> u32 {
    let start_height = tree_map[pos.0 as usize][pos.1 as usize];
    let mut visible_count = 0;
    loop {
        pos.0 += dir.0;
        pos.1 += dir.1;
        visible_count += 1;
        if pos.0 == 0 || pos.1 == 0 || pos.0 == tree_map.len() as i32 -1 || pos.1 == tree_map.len() as i32 -1 {
            break;
        }
        if tree_map[pos.0 as usize][pos.1 as usize] >= start_height {
            break;
        } 
    }
    return visible_count;
}


fn main() {
    let input = fs::read_to_string("inputs/aoc8.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>(); 
    let width = lines[0].len();
    let height= lines.len();
    println!( "width: {}, height: {}", width, height);

    let mut tree_map: Vec<Vec<u8>> = Vec::new();          // row - > columne order (y.x)
    for line in lines {
        tree_map.push( line.as_bytes().to_owned() );
    }

    // Part 1
    let mut tree_mask = vec![vec![0 as u32;width];height];     // row - > columne order (y.x)
    for y in 0..height{
        let mut highest_seen = [-1 as i8;4];
        for x in 0..width {
            update_scan( x,y, &mut highest_seen[0], &mut tree_mask, &tree_map );
            update_scan( width-x-1,y, &mut highest_seen[1], &mut tree_mask, &tree_map );
            update_scan( y, x, &mut highest_seen[2], &mut tree_mask, &tree_map );
            update_scan( y, height-x-1, &mut highest_seen[3], &mut tree_mask, &tree_map );
        }
    }
    let mut total = 0;
    for y in 0..height{
        for x in 0..width{
            total += tree_mask[y][x]; 
        }
    }
    println!( "total visible: {}", total);

    // Part 2
    let mut best_score = 0;
    for y in 1..height-1{
        let mut highest_seen = [-1 as i8;4];
        for x in 1..width-1 {
            let start_pos = (y as i32, x as i32 );
            let mut score =  cast_ray( start_pos, (0,-1), &tree_map );
            score  *= cast_ray( start_pos, (0,1), &tree_map );
            score  *= cast_ray( start_pos, (1,0), &tree_map );
            score  *= cast_ray( start_pos, (-1,0), &tree_map );
            if score > best_score {
                best_score = score;
            }
        }
    }
    println!( "Best score: {}", best_score);
}