use std::{fs, collections::{BinaryHeap, HashMap}};

fn main() {
    // Part 1
    let input = fs::read_to_string("inputs/aoc2.txt").unwrap();
    let shape_score_map = HashMap::from( [ ("X",1 ), ("Y",2),("Z",3)]);
    let game_score_map = HashMap::from( [ 
        ("AX",3 ), ("AY",6),("AZ",0),           //rock (a,x)  paper (b,y),   scissors (c,z)
        ("BX",0 ), ("BY",3),("BZ",6),
        ("CX",6 ), ("CY",0),("CZ",3),
        ]);
    let mut total_score = 0;
    for line in input.lines() {
        let mut parts: Vec<&str> = line.split(' ').collect();//.split(' ');//.collect();
        let shape_score = shape_score_map.get(parts[1]).unwrap();
        let game =  parts[0].to_string() + parts[1];
        let game_score = game_score_map.get(&game[..]).unwrap();
        total_score += game_score + shape_score; 
    }
    println!( "Total score {}", total_score);

    // Part 2
    let input = fs::read_to_string("inputs/aoc2.txt").unwrap();
    // Given elf piece, and desired outcomr. Tells what piece we need t pick to get that
    let strategy_map =  HashMap::from( [
        ("AX","Z" ), ("AY","X"), ("AZ","Y"),           //rock (a,x)  paper (b,y),   scissors (c,z)
        ("BX","X" ), ("BY","Y"), ("BZ","Z"),           // x=> lose, y=> draw,  z =>win
        ("CX","Y" ), ("CY","Z"), ("CZ","X")
    ]);
    let strategy_score = HashMap::from( [ ("X", 0), ("Y", 3), ("Z", 6)]);
    let mut total_score = 0;
    for line in input.lines() {
        let mut parts: Vec<&str> = line.split(' ').collect();//.split(' ');//.collect();
        let strategy_key = parts[0].to_string() + parts[1];
        println!( "strategy_key {}", strategy_key);
        let my_strategy = *strategy_map.get(&strategy_key[..]).unwrap();
        let shape_score = shape_score_map.get(my_strategy).unwrap();
        let strategy_score = strategy_score.get( parts[1]).unwrap();

        println!( "{}:   {},  ({} + {} = {})", parts[0], parts[1], shape_score, strategy_score, shape_score+strategy_score);
        total_score += strategy_score + shape_score; 
    }
    println!( "Total score {}", total_score);

}
