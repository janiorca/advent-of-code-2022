use std::{fs, collections::{HashSet, HashMap}};

fn generate_spans( sensors: &Vec<((i64,i64), i64)>, scan_y: i64 ) -> Vec<(i64,i64)> {
    let mut spans: Vec<(i64,i64)> = Vec::new();

    for (sensor_pos, sensor_range) in sensors {
        let y_dist = (sensor_pos.1-scan_y).abs();
        let cover = sensor_range - y_dist; 
        if cover>= 0 {
            let start = (sensor_pos.0-cover).max(0);
            let stop = sensor_pos.0+cover.min(4000000);
            // check if clipping deleted the span
            if start > stop {
                continue;
            } 
            if spans.len() == 0 {
                spans.push( (start, stop));
            } else {
                for index in 0..spans.len() {
                    if stop + 1 < spans[index].0 {
                        // The new span preceeds all others. Insert it at front
                        spans.insert(index, (start,stop));
                        break;
                    } else if spans[index].1 + 1 < start {
                        // the new span is after the current, keep searching unless it was the last one
                        if index == spans.len()-1 {
                            spans.push((start,stop));
                        } 
                    } else {
                        // still here, combine at least the current and next one.
                        let mut end_index = index;
                        while end_index+1 < spans.len() && stop + 1 >= spans[end_index+1].0 {
                            end_index += 1;
                        }
                        spans[index].0 = start.min( spans[index].0);
                        spans[index].1 = stop.max( spans[end_index].1);
                        for _t in 0..end_index - index {
                            spans.remove(index+1);
                        }
                        break;
                    }
                }
            }
        }
    }
    return spans;
}
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

    // Part1 ( extremely simple and extremely slow )
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

    // part 2
    for y in 0..4000000 {
        let spans = generate_spans(&sensors, y);
        if spans[0].0 > 1 {     
            // at the front
            println!( "tuning freq: {} ", y);
        } else if spans[0].1 < 4000000 {
            // between two spans or at the end of first ( either case end of span 0 is less than 4000000)
            println!( "tuning freq: {} ", (spans[0].1+1)*4000000+y);
            break;
        }
    }
}