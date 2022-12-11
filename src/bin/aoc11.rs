use std::{fs, collections::{HashSet, VecDeque, HashMap}, hash::Hash};

struct Monkey{
    held_items: VecDeque<u64>,
    operator: String,
    operand: u64,
    modulo_check: u64,
    true_dest: u64,
    false_dest: u64,
    inspection_count: u64
}

impl Monkey {
    fn operate( &mut self, value: u64, relief: u64 ) -> (u64, u64) {
        self.inspection_count += 1;
        let mut value = match self.operator.as_str() {
            "+" => value + self.operand,
            "*" => value * self.operand,
            "^" => value * value,
            _ => panic!("Unexpected operator")
        };
        value /= relief;
        if value % self.modulo_check == 0 {
            return (self.true_dest, value);
        } else {
            return (self.false_dest, value);
        }
    }
}
fn tokenize( string: &str ) -> Vec<&str> {
    let tokens: Vec<&str> = string.split(&[' ',':', ','][..]).filter(|x| *x != "" ).collect();
    return tokens;
}

fn get_monkey_data() -> Vec<Monkey> {
   // Parse the monkey data
   let input = fs::read_to_string("inputs/aoc11.txt").unwrap();
   let mut input_lines = input.lines().peekable();
   let mut monkeys = Vec::new();
   while input_lines.peek().is_some() {
       let monkey_string: Vec<&str> = tokenize( input_lines.next().unwrap());
       println!( "Reading Monkey {} {}", monkey_string[0], monkey_string[1] );

       let items_string: Vec<&str> = tokenize( input_lines.next().unwrap());
       let items: VecDeque<u64> = items_string[2..].iter().map(|x| x.parse::<u64>().unwrap() ).collect();

       let op_strings: Vec<&str> = tokenize( input_lines.next().unwrap());
       let (operator, operand) =  
           if op_strings[5] == "old" { 
               ( "^", 2)   } 
           else {
               (op_strings[4], op_strings[5].parse::<u64>().unwrap()) 
           };

       let mod_strings: Vec<&str> = tokenize( input_lines.next().unwrap());
       let modulo_check = mod_strings[3].parse::<u64>().unwrap();

       let true_dest = tokenize( input_lines.next().unwrap())[5].parse::<u64>().unwrap();
       let false_dest = tokenize( input_lines.next().unwrap())[5].parse::<u64>().unwrap();
       input_lines.next();

       monkeys.push(Monkey { held_items: items, operator: operator.to_string(), operand: operand, modulo_check, true_dest, false_dest, inspection_count:0});
   }
   return monkeys;
}

fn calculate_monkey_business( mut monkeys: Vec<Monkey>, relief: u64, rounds: i64 ) -> u64 {
    let monkey_modulo = monkeys.iter().map(|x| x.modulo_check ).reduce(|x,y| x*y ).unwrap();
    for _round in 0..rounds {
        for monkey_no in 0..monkeys.len() {
            while let Some(item) = monkeys[monkey_no].held_items.pop_front() {
                let (dest, value) = monkeys[monkey_no].operate( item, relief );
                monkeys[(dest) as usize].held_items.push_back(value % monkey_modulo);
            }
        }
    }

    let mut counts: Vec<u64> = monkeys.iter().map(|x| x.inspection_count).collect();
    counts.sort();
    counts.reverse();
    return counts[0]*counts[1]
}

fn main() {
    let mut monkeys = get_monkey_data();
    println!("{} monkey business", calculate_monkey_business(monkeys, 3, 20));

    let mut monkeys = get_monkey_data();
    println!("{} HArd monkey business", calculate_monkey_business(monkeys, 1, 10000));
}
