use std::{fs, collections::HashSet};

struct CRT{
    current_line: String,
    horizontal_pos: i64
}

impl CRT {
    fn cycle( &mut self, new_sprite_pos: i64 ) {
        let new_char = if (new_sprite_pos-self.horizontal_pos ).abs() <= 1 { '#' } else { '.'};
        self.current_line.push(new_char);
        self.horizontal_pos += 1;
        if self.horizontal_pos == 40 {
            self.horizontal_pos =0;
            println!("{}", self.current_line);
            self.current_line.clear();
        }
    }
}
struct Register{
    crt: CRT,
    value: i64,
    sampled_values: i64,
    current_cycle: i64
}

impl Register{
    fn cycle(&mut self) {
        self.current_cycle += 1;
        self.crt.cycle(self.value);
        if (self.current_cycle + 20)%40 == 0 {
            self.sampled_values += self.value*self.current_cycle;
        } 
    }
}

fn main() {
    // Part1 & Part2
    let input = fs::read_to_string("inputs/aoc10.txt").unwrap();
    let mut crt = CRT{ current_line: "".to_string(),  horizontal_pos: 0};
    let mut register = Register{ crt, value: 1, sampled_values:0, current_cycle:0 };
    for line in input.lines() {
        let instruction_parts: Vec<&str> = line.split(' ').collect();

        match instruction_parts[0] {
            "noop" => register.cycle(),
            "addx" => {
                let operand = instruction_parts[1].parse::<i64>().unwrap(); 
                register.cycle();
                register.cycle();
                register.value += operand;
            },
            _ => panic!("unexpected instruction")
        }
    }
    println!( "total: {} ", register.sampled_values);
}