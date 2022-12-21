use std::fs;

#[derive(Debug)]
struct InputNumber{
    value: i64,
    original_pos: usize    
}

fn get_numbers( scale: i64 ) -> Vec<InputNumber> {
    let input = fs::read_to_string("inputs/aoc20.txt").unwrap();
    let mut numbers: Vec<InputNumber> = Vec::new();
    for (original_pos, input_line)  in input.lines().enumerate() {
        let value = input_line.parse::<i64>().unwrap();
        numbers.push( InputNumber{ value: value*scale, original_pos });
    }
    return numbers;
}

fn decrypt( mut numbers: Vec<InputNumber>, cycles: i64) -> i64 {
    let message_size = numbers.len() as i64 - 1 ;
    for _mixing_round in 0..cycles{
        for current in 0..numbers.len() {
            let index = numbers.iter().position(|x| x.original_pos == current ).unwrap();
            let mut new_index = index as i64 + numbers[ index ].value;
            new_index = ((new_index % message_size) + message_size) % message_size;
            let number = numbers.remove(index);
            numbers.insert( new_index as usize, number);
        }
    }

    let zero_index = numbers.iter().position(|x| x.value == 0 ).unwrap();
    let n1 = numbers[ (zero_index+1000)%numbers.len()].value;
    let n2 = numbers[ (zero_index+2000)%numbers.len()].value;
    let n3 = numbers[ (zero_index+3000)%numbers.len()].value;
    return n1+n2+n3;
}

fn main() {
    // Part 1 
    let numbers = get_numbers(1);
    let result = decrypt(numbers, 1);
    println!( "{} result", result);

    // Part 2
    let numbers = get_numbers(811_589_153);
    let result = decrypt(numbers, 10);
    println!( "{} result", result);
}
