use std::{fs, collections::HashMap};

#[derive(Clone) ]
enum Operation{
    Add,
    Subtract,
    Multiply,
    Divide
}
#[derive(Clone) ]
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
        OperationType::Unresolved { operation: _, left: _, right: _ } => None
    }
}

fn evaluate( mut ops: HashMap<String, OperationType > ) -> i64 {
    loop{
        let names = ops.keys().cloned().collect::<Vec<String>>();
        for name in names {
            // Are the operands resolved
            let operation_type = ops.get(&name).unwrap();
            match operation_type {
                OperationType::Resolved{ value: _ } => continue,
                OperationType::Unresolved { operation, left, right } => {
                    let left_value = eval_operand( ops.get(left).unwrap());
                    let right_value = eval_operand( ops.get(right).unwrap());
                    if left_value.is_some() && right_value.is_some() {
                        let result = match operation {
                            Operation::Add => left_value.unwrap() + right_value.unwrap(),                            
                            Operation::Subtract => left_value.unwrap() - right_value.unwrap(),                            
                            Operation::Divide => left_value.unwrap() / right_value.unwrap(),                            
                            Operation::Multiply => left_value.unwrap() * right_value.unwrap(),                            
                        };
                        ops.insert(name, OperationType::Resolved { value: result });
                    }
                }
            }
        }
        if let OperationType::Resolved { value } = ops.get("root").unwrap() {
            return *value;
        } 
    }
}

fn evaluate_with_value( mut ops: HashMap<String, OperationType >, value: i64 ) -> i64 {
    ops.insert("humn".to_string(), OperationType::Resolved { value });

    return evaluate(ops);
}

fn solve( mut ops: HashMap<String, OperationType > ) -> i64 {
    // Tweak the human op to be - , that way we solve for 0-root 
    let root = ops.remove("root").unwrap();
    if let OperationType::Unresolved { operation: _, left, right } = root {
        ops.insert("root".to_string(), OperationType::Unresolved { operation: Operation::Subtract, left: left.clone(), right: right.clone() } );
    } else {
        panic!( "cant update root")
    }

    // // find initial range
    let mut lp = (0, evaluate_with_value(ops.clone(), 0) );
    let mut rp = (1,evaluate_with_value(ops.clone(), 1) );
    while  lp.1.signum() == rp.1.signum()  {
        lp = rp;
        rp = ( lp.0*2, evaluate_with_value(ops.clone(), lp.0*2));
    }

    // use the bisection method to find the root
    loop{
        let mid = (lp.0 + rp.0 )/2;
        let mp = ( mid, evaluate_with_value(ops.clone(), mid) );
        if mp.1.signum() != lp.1.signum() { 
            rp = mp;
        } else {
            lp = mp;
        }
        if lp.1==0 { return lp.0 }
        if rp.1==0 { return rp.0 }
    }
}

fn main() {
    let monkey_ops = get_monkeys();
    // Part 1
    let result = evaluate(monkey_ops.clone());
    println!( "evaluates: {}", result);

    // Part 2
    let result = solve(monkey_ops.clone());
    println!( "part 2 root: {}", result);
}
