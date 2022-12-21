use std::{fs, collections::HashMap};

enum Operation{
    Add,
    Subtract,
    Multiply,
    Divide
}
enum OperationType{
    Resolved{ value: i64  },
    Unresolved{ operation: Operation, left: String, right: String},
}

fn get_monkeys() -> HashMap<String, OperationType > {
    let input = fs::read_to_string("inputs/aoc21.txt").unwrap();
    let mut monkey_ops: HashMap<String, OperationType > = HashMap::new();
    for line in input.lines() {
        let cleaned = line.replace(":", "");
        let parts = cleaned.split(' ').collect::<Vec<&str>>();
        let name: String = parts[0].to_owned();
        if parts.len() == 2 {
            let value = parts[1].parse::<i64>().unwrap();
            monkey_ops.insert(name, OperationType::Resolved {  value });
        } else {
            let left = parts[1].to_owned();
            let right = parts[3].to_owned();
            let operation = match parts[2] {
                "+" =>  OperationType::Unresolved { operation: Operation::Add, left, right},
                "-" =>  OperationType::Unresolved { operation: Operation::Subtract, left, right},
                "/" =>  OperationType::Unresolved { operation: Operation::Divide, left, right},
                "*" =>  OperationType::Unresolved { operation: Operation::Multiply, left, right},
                _ => panic!( "unknown operator")
            };
            monkey_ops.insert(name, operation);
        }
    }
    return monkey_ops;
}

fn eval_operand( operand: &OperationType ) -> Option<i64> {
    return match operand {
        OperationType::Resolved{ value } => Some( *value ),
        OperationType::Unresolved { operation, left, right } => None
    }
}

fn main() {
    let mut monkey_ops = get_monkeys();

    loop{
        let names = monkey_ops.keys().cloned().collect::<Vec<String>>();
        for name in names {
            // Are the operands resolved
            let operation_type = monkey_ops.get(&name).unwrap();
            match operation_type {
                OperationType::Resolved{ value } => continue,
                OperationType::Unresolved { operation, left, right } => {
                    let left_value = eval_operand( monkey_ops.get(left).unwrap());
                    let right_value = eval_operand( monkey_ops.get(right).unwrap());
                    if left_value.is_some() && right_value.is_some() {
                        let result = match operation {
                            Operation::Add => left_value.unwrap() + right_value.unwrap(),                            
                            Operation::Subtract => left_value.unwrap() - right_value.unwrap(),                            
                            Operation::Divide => left_value.unwrap() / right_value.unwrap(),                            
                            Operation::Multiply => left_value.unwrap() * right_value.unwrap(),                            
                        };
                        monkey_ops.insert(name, OperationType::Resolved { value: result });
                    }
                }
            }
        }
        if let OperationType::Resolved { value } = monkey_ops.get("root").unwrap() {
            println!( "ROOT: {}", value);
            break;
        } 
    }

}
