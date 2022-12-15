use std::{fs, collections::HashSet};

//fn get_number
fn main() {
    let input = fs::read_to_string("inputs/aoc15.txt").unwrap();
    let lines = input.lines();
    let mut sensors: Vec<((i64,i64), i64)> = Vec::new();
    for line in lines{
        let cleaned = line.to_owned().replace(",", "").replace(":", "");
        let parts: Vec<&str> = cleaned.split(&[' ', '='][..]).collect();
        let sensor_pos = (parts[3].parse::<i64>().unwrap(),parts[5].parse::<i64>().unwrap());
        let beacon_pos = (parts[11].parse::<i64>().unwrap(),parts[13].parse::<i64>().unwrap());
        let dist = (sensor_pos.0 - beacon_pos.0).abs() + (sensor_pos.1 - beacon_pos.1).abs(); 
        sensors.push( ((sensor_pos.0, sensor_pos.1), dist));
    }

    // Part1
    let beacon_y = 2000000;
    let mut covered_x: HashSet<i64> = HashSet::new();
    for (sensor_pos, sensor_range) in &sensors {
        let y_dist = (sensor_pos.1-beacon_y).abs();
        let cover = sensor_range - y_dist; 
        if cover>= 0 {
            for x in sensor_pos.0-cover..sensor_pos.0+cover {
                covered_x.insert(x);
            }
        }
    }
    println!( "covered areas {} at {}", covered_x.len(), beacon_y); 
}