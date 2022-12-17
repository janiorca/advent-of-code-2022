use std::{fs, collections::{HashSet, HashMap}};

struct Node{
    flow_rate: i64,
    direct_links: Vec<String>,
    travel_costs: HashMap<String,i64>
}
#[derive(Clone,Copy)] 
struct Searcher<'a>{
    node_name: &'a String,
    remaining_turns: i64
}

fn get_best_path_value( remaining_dest: &HashSet<&String>, nodes: &HashMap<String,Node>, current_node_name: &String, mut remaining_turns: i64 ) -> i64 {
    let current_node = nodes.get(current_node_name).unwrap();
    let node_value;

    if current_node.flow_rate > 0 {
        remaining_turns -=1;
        node_value = current_node.flow_rate*remaining_turns;
    } else {
        node_value = 0;
    }
    let mut inner_remaining_dest = remaining_dest.clone();
    inner_remaining_dest.remove(current_node_name);
    let inner_remaining_dest = inner_remaining_dest;

    let mut max_inner_value = 0;
    for inner_node_name in &inner_remaining_dest {
        let travel_cost = *current_node.travel_costs.get(*inner_node_name).unwrap();
        if travel_cost < remaining_turns {
            let inner_value = get_best_path_value( &inner_remaining_dest, nodes, inner_node_name, remaining_turns - travel_cost);
            max_inner_value = inner_value.max(max_inner_value);
        }
    }
    return max_inner_value + node_value;
}

// Update multiple searchers simultaneously
fn get_best_path_value_multi( remaining_dest: &HashSet<&String>, nodes: &HashMap<String,Node>, searchers: &mut [Searcher;2]) -> i64 {
    // which one are we updating
    let current_searcher = if searchers[0].remaining_turns > searchers[1].remaining_turns { 0 } else { 1 };
    
    let current_node = nodes.get(searchers[current_searcher].node_name).unwrap();
    let mut node_value;
    if current_node.flow_rate > 0 {
        searchers[current_searcher].remaining_turns -=1;
        node_value = current_node.flow_rate*searchers[current_searcher].remaining_turns;
    } else {
        node_value = 0;
    }

    let mut max_inner_value = 0;
    if remaining_dest.len() > 0 {
        for inner_node_name in remaining_dest {
            let travel_cost = *current_node.travel_costs.get(*inner_node_name).unwrap();
            if travel_cost < searchers[current_searcher].remaining_turns {
                let mut inner_searchers = searchers.clone();
                inner_searchers[ current_searcher ].remaining_turns -= travel_cost;
                inner_searchers[ current_searcher ].node_name = inner_node_name;
                let mut inner_remaining_dest = remaining_dest.clone();
                inner_remaining_dest.remove(inner_node_name);

                let inner_value = get_best_path_value_multi( &inner_remaining_dest, nodes, &mut inner_searchers);
                max_inner_value = inner_value.max(max_inner_value);
            }
        }
        // At times it makes more sense to just stop ( and let the other searcher to visit the remaining parts)
        if searchers[ current_searcher ].remaining_turns > 1 {
            let mut inner_searchers = searchers.clone();
            inner_searchers[ current_searcher ].remaining_turns = 1;
            let aa = "AA".to_string();
            inner_searchers[ current_searcher ].node_name = &aa;
            let inner_value = get_best_path_value_multi( &remaining_dest, nodes, &mut inner_searchers);
            max_inner_value = max_inner_value.max(inner_value);
        }         
    }
    if remaining_dest.len() == 0 || max_inner_value == 0 {
        // we couldnt visit any other destination or there are none left. Finish up the outstanding search
        let other_searcher = if current_searcher == 1  { 0 } else { 1 };
        let other_node = nodes.get(searchers[other_searcher].node_name).unwrap();
        node_value += other_node.flow_rate*(searchers[other_searcher].remaining_turns-1);
    }
    return max_inner_value + node_value;
}


fn main() {
    let input = fs::read_to_string("inputs/aoc16.txt").unwrap();
    let mut nodes: HashMap<String,Node> = HashMap::new();
    // Get input and turn node map
    for line in input.lines(){
        let cleaned = line.to_owned().replace("=", " ").replace(",", "").replace(";", "");
        let tokens = cleaned.split_whitespace().collect::<Vec<&str>>();
        let node_name = tokens[1];
        let flow_rate = tokens[5].parse::<i64>().unwrap(); 
        let direct_links = tokens[10..].iter().map(|x|(*x).to_owned()).collect::<Vec<String>>(); 
        nodes.insert(node_name.to_string(), Node{flow_rate, direct_links,travel_costs:HashMap::new()});
    }

    // Work out all node-node travel costs in advance ( makes the search MUCH simpler)
    let all_nodes: HashSet<&String> = HashSet::from_iter(nodes.keys());
    let mut all_travel_costs: HashMap<String,HashMap<String,i64>> = HashMap::new();
    {
        for node_name  in nodes.keys().cloned() {
            let mut travel_costs: HashMap<String, i64> = HashMap::from([(node_name.to_owned(),0)]);

            let mut travel_cost = 0;
            while travel_costs.len() < all_nodes.len() {
                let src_node_names: Vec<String> = travel_costs.iter().filter(|x| *x.1==travel_cost ).map(|x| x.0.to_string()).collect();
                travel_cost += 1;
                for src_name in src_node_names {
                    // all the nodes we can get to that have not yet been visited
                    for link in &(nodes.get(&src_name).unwrap().direct_links) {
                        if !travel_costs.contains_key(link) {
                            travel_costs.insert(link.to_string(), travel_cost);
                        }
                    }
                }
            }
            all_travel_costs.insert(node_name.to_string(), travel_costs);
        }
    }

    // Update all nodes with the travel costs
    for (node_name, travel_costs) in all_travel_costs {
        let node = nodes.get_mut(&node_name).unwrap();
        node.travel_costs = travel_costs;
    }

    // We only care about nodes with a functioning value. All other nodes are effectively graph edges ( no decision needs to happen on them)
    let mut remaining_dests: HashSet<&String> = HashSet::new();
    for (node_name, node ) in &nodes {
        if node.flow_rate > 0 {
            remaining_dests.insert(&node_name);
        } 
    }

    let best = get_best_path_value( &remaining_dests, &nodes, &"AA".to_string(), 30);
    println!( "Part 1 solution is {}", best);

    let aa = "AA".to_string();
    let mut searchers = [ Searcher{node_name: &aa, remaining_turns: 26}; 2];
    let best2 = get_best_path_value_multi( &remaining_dests, &nodes, &mut searchers);
    println!( "Part 2 solution is {}", best2);
}

