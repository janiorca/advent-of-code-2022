use std::fs;

fn main() {
    // Part1 and Part2
    let input = fs::read_to_string("inputs/aoc4.txt").unwrap();
    let mut contained = 0;
    let mut overlap = 0;
    for line in input.lines() {
        let parts = line.split( &[ ',', '-'][..] ).collect::<Vec<&str>>();
        let values = parts.iter().map(|x|u32::from_str_radix(x, 10).unwrap()).collect::<Vec<u32>>();
        // containment for part 1 
        if ( values[0] <= values[2] && values[1] >= values[3] ) || ( values[0] >= values[2] && values[1] <= values[3] ) {
            contained += 1;
        }
        // overlap for part 2 
        if values[1] >= values[2] && values[3] >= values[0] {
            overlap += 1;
        }

    }
    println!("Contained: {}, Overlap: {}", contained, overlap);

}
