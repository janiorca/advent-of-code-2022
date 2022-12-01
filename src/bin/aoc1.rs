use std::{fs, collections::BinaryHeap};

fn main() {
    // Part1 & Part2
    let input = fs::read_to_string("inputs/aoc1.txt").unwrap();
    let mut elf_calorie_heap: BinaryHeap<u64> = BinaryHeap::new();
    let mut elf_calories = 0;
    for line in input.lines() {
        if line.len() == 0  {
            elf_calorie_heap.push(elf_calories);
            elf_calories = 0;
        } else {
            let new_value = u64::from_str_radix(line,10).unwrap();
            elf_calories += new_value;
        }
    }

    println!("Max elf calories: {}", elf_calorie_heap.peek().unwrap());
    let top3_total = elf_calorie_heap.pop().unwrap() + elf_calorie_heap.pop().unwrap() + elf_calorie_heap.pop().unwrap();
    println!("Total top 3  elf calories: {}", top3_total);

}
