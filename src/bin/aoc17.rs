use std::fs;

struct Rect{ 
    x: i32,
    y: i32,
    width: i32,
    height: i32
}

fn get_rock_shapes() -> Vec<Vec<Rect>> {
    return vec![ 
        vec![ Rect{x:0, y:0, width:4, height:1}],
        vec![ Rect{x:1, y:0, width:1, height:3}, Rect{x:0, y:1, width:3, height:1} ],
        vec![ Rect{x:0, y:0, width:3, height:1}, Rect{x:2, y:0, width:1, height:3} ],
        vec![ Rect{x:0, y:0, width:1, height:4} ],
        vec![ Rect{x:0, y:0, width:2, height:2} ],
    ];
}

fn intersects( tower: &Vec<[u8;7]>, rock: &Vec<Rect>, rock_pos: (i32,i32)) -> bool {
    for rect in rock {
        for x in rect.x..(rect.x+rect.width){
            for y in rect.y..(rect.y+rect.height){
                let check_pos = (x+rock_pos.0, y+rock_pos.1);
                if check_pos.0 < 0 || check_pos.0 >= 7 || check_pos.1 < 0 {
                    return true;
                }
                if check_pos.1 < tower.len() as i32 {
                    if tower[check_pos.1 as usize][check_pos.0 as usize] != 0 {
                        return true;
                    }
                }
            }
        }
    }
    return false;
}

fn place_rock( tower: &mut Vec<[u8;7]>, rock: &Vec<Rect>, rock_pos: (i32,i32)) -> bool {
    for rect in rock {
        for x in rect.x..(rect.x+rect.width){
            for y in rect.y..(rect.y+rect.height){
                let place_pos = (x+rock_pos.0, y+rock_pos.1);
                if place_pos.1 >= tower.len() as i32 {
                    tower.push([0u8;7]);
                }
                tower[place_pos.1 as usize][place_pos.0 as usize] = 1;
            }
        }
    }
    return false;
}

fn main() {
    let input = fs::read_to_string("inputs/aoc17.txt").unwrap();
    let mut jet = input.chars().cycle();
    let mut rocks = [0,1,2,3,4].iter().cycle();
    let mut tower: Vec<[u8;7]> = Vec::new();
    let rock_shapes = get_rock_shapes();

    for _rock_no in 0..2022 {
        // Create the rock
        let mut rock_pos = (2i32, tower.len() as i32 + 3); 
        let rock_index = *rocks.next().unwrap();

        loop{
            // apply jet
            let potential_x = rock_pos.0 + if jet.next().unwrap() == '<' { -1 } else { 1 };
            if !intersects(&tower, &rock_shapes[ rock_index ], (potential_x, rock_pos.1 )) {
                rock_pos.0 = potential_x;
            }

            // fall
            if !intersects(&tower, &rock_shapes[ rock_index ], (rock_pos.0, rock_pos.1-1)) {
                rock_pos.1 -=1;
            } else {
                // if at rest, add the rock to the pile and move on
                place_rock(&mut tower, &rock_shapes[ rock_index ], rock_pos);
                break;
            }
        }
    }
    println!( "tower height: {}", tower.len());
}
