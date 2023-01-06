use std::{collections::{HashMap, VecDeque}, fs};

fn u64_to_snafu( mut value: u64 ) -> String {
    let mut result: VecDeque<char> = VecDeque::new();
    let chars: Vec<char> = vec![ '0', '1', '2', '=', '-' ];
    while value > 0 {
        let rem = value % 5;
        result.push_front( chars[rem as usize ]);
        value = value / 5;
        if rem > 2 {
            value += 1;
        }
    }
    return result.iter().collect::<String>();
}

fn snafu_to_u64( snafu_value: &str ) -> u64 {
    let digit_map: HashMap<char, i64> = HashMap::from([ ('2', 2), ('1', 1), ('0', 0), ('-', -1), ('=', -2) ]);
    let mut value = 0i64;
    let mut place_value = 1i64;
    let mut chars: Vec<char> = snafu_value.chars().collect();
    chars.reverse();
    for ch in chars {
        value += digit_map.get(&ch).unwrap()*place_value;
        place_value *= 5;
    }
    return value as u64;
}

fn main() {
    let input = fs::read_to_string("inputs/aoc25.txt").unwrap();
    let mut snafu_sum = 0u64;
    for line in input.lines() {
        snafu_sum += snafu_to_u64(line);
    }
    println!( "snafu sum {},  in snafu [ {} ]",snafu_sum, u64_to_snafu(snafu_sum as u64) );
}