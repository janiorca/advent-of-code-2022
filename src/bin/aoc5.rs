use std::fs;

struct Move{
    num_items: u32,
    source: u32,
    destination: u32,
}

fn get_input() -> (Vec<Vec<char>>, Vec<Move>) {
    let input = fs::read_to_string("inputs/aoc5.txt").unwrap();
    // read starting state
    let mut columns: Vec<Vec<char>> = Vec::new();
    let mut moves: Vec<Move> = Vec::new();

    for line in input.lines() {
        if line.contains("[") {
            // reading column data
            let column_count = (line.len()+1)/4;
            if columns.len() == 0 {
                for _count in 0..column_count {
                    columns.push( Vec::<char>::new() );
                }
            }
            for column_idx in 0..column_count {
                let c = line.as_bytes()[ column_idx*4+1] as char;
                if c != ' ' {
                    columns[column_idx].push( c );
                }
            }
        } else if line.contains("move") {
            // reading moves data. We can assume that the columns are fully created at this point
            let command_parts = line.split( ' ').collect::<Vec<&str>>();
            let num_items = command_parts[1].parse::<u32>().unwrap();
            let source = command_parts[3].parse::<u32>().unwrap()-1;
            let destination = command_parts[5].parse::<u32>().unwrap()-1;
            moves.push(Move{num_items, source,destination});
        } else if line.len() == 0 {
            println!( "Starting moves" );
            // Reverse the columns which are 'upside-down' beacuse we created them top-first
            for column in &mut columns{
                column.reverse();
            }
        }
    }
    return (columns, moves);
}

fn main() {
    // Part1 
    let (mut columns, moves) = get_input();
    for move_command in moves {
        for _count in 0..move_command.num_items {
            let item = (&mut columns[ move_command.source as usize]).pop().unwrap();
            (&mut columns[ move_command.destination as usize]).push( item );
        }
    }
    for column in &columns{
        print!( "{}", column.last().unwrap());
    }

    // Part2 
    let (mut columns, moves) = get_input();
    for move_command in moves {
        let mut temp: Vec<char> = Vec::new();        
        for _count in 0..move_command.num_items {
            temp.push( (&mut columns[ move_command.source as usize]).pop().unwrap() );
        }
        for _count in 0..move_command.num_items {
            let item = temp.pop().unwrap();
            (&mut columns[ move_command.destination as usize]).push( item );
        }
    }
    for column in &columns{
        print!( "{}", column.last().unwrap());
    }

}
