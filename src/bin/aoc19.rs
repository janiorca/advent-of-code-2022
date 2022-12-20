use std::fs;

#[derive(Debug)]
struct Blueprint{
    ore_robot_cost: i32,
    clay_robot_cost: i32,
    obsidian_robot_cost_ore: i32,
    obsidian_robot_cost_clay: i32,
    geode_robot_cost_ore: i32,
    geode_robot_cost_obsidian: i32,

    max_ore_use_per_turn: i32,
}

#[derive(PartialEq,Clone,Eq, Hash)]
enum Construction{
    OreRobot,
    ClayRobot,
    ObsidianRobot,
    GeodeRobot
}

#[derive(Clone,PartialEq, Eq,Hash)]
struct State{
    ore: i32,
    clay: i32,
    obsidian: i32,
    geodes: i32,

    ore_robots: i32,    
    clay_robots: i32,
    obsidian_robots: i32,
    geode_robots: i32,

    construction: Construction
}

fn find_most_geodes( blueprint: &Blueprint, mut remaining_turns: i32, mut state: State ) -> i32 {
    let mut robot_constructed = false;
    while robot_constructed == false && remaining_turns > 0 {
        match state.construction {
            Construction::OreRobot => {
                if state.ore >= blueprint.ore_robot_cost {
                    state.ore -= blueprint.ore_robot_cost;
                    robot_constructed = true;
                }
            },
            Construction::ClayRobot => {
                if state.ore >= blueprint.clay_robot_cost {
                    state.ore -= blueprint.clay_robot_cost;
                    robot_constructed = true;
                }
            }, 
            Construction::ObsidianRobot => {
                if state.ore >= blueprint.obsidian_robot_cost_ore && state.clay >= blueprint.obsidian_robot_cost_clay {
                    state.ore -= blueprint.obsidian_robot_cost_ore;
                    state.clay -= blueprint.obsidian_robot_cost_clay;
                    robot_constructed = true;
                }
            }, 
            Construction::GeodeRobot => {
                if state.ore >= blueprint.geode_robot_cost_ore && state.obsidian >= blueprint.geode_robot_cost_obsidian {
                    state.ore -= blueprint.geode_robot_cost_ore;
                    state.obsidian -= blueprint.geode_robot_cost_obsidian;
                    robot_constructed = true;
                }
            }
        }
        state.ore += state.ore_robots;
        state.clay += state.clay_robots;
        state.obsidian += state.obsidian_robots;
        state.geodes += state.geode_robots;
        remaining_turns -= 1;
        if robot_constructed {
            match state.construction{
                Construction::OreRobot => { state.ore_robots += 1 },
                Construction::ClayRobot => { state.clay_robots += 1 },
                Construction::ObsidianRobot => { state.obsidian_robots += 1 }, 
                Construction::GeodeRobot => { state.geode_robots += 1 }
            }
        }
    }

    // Decide what do build next. Always possible to build ore and clay robots but 
    let mut max_geodes = state.geodes;
    if remaining_turns > 0 {
        for next_robot in [ Construction::OreRobot, Construction::ClayRobot, Construction::ObsidianRobot, Construction::GeodeRobot ] {
            // Dont evaluate paths that have unbuildablle robots
            if next_robot == Construction::ObsidianRobot && state.clay_robots == 0 { 
                continue;
            }
            if next_robot == Construction::GeodeRobot && state.obsidian_robots == 0 { 
                continue;
            }
            // Dont build more robots for a resource than could be consumed
            if next_robot == Construction::OreRobot && state.ore_robots == blueprint.max_ore_use_per_turn ||
                next_robot == Construction::ClayRobot && state.clay_robots == blueprint.obsidian_robot_cost_clay ||
                next_robot == Construction::ObsidianRobot && state.obsidian_robots == blueprint.geode_robot_cost_obsidian
            {
                continue;
            }

            let mut search_state = state.clone();
            search_state.construction = next_robot;
            let num_geodes = find_most_geodes( blueprint, remaining_turns, search_state);
            max_geodes = max_geodes.max(num_geodes);
        }
    }
    return max_geodes;
}

fn main() {
    // Part1
    let input = fs::read_to_string("inputs/aoc19.txt").unwrap();
    let mut blueprints: Vec<Blueprint> = Vec::new();
    for line in input.lines() {
        let parts = line.split(' ').collect::<Vec<&str>>();
        let ore_robot_cost = parts[6].parse::<i32>().unwrap();
        let clay_robot_cost = parts[12].parse::<i32>().unwrap();
        let obsidian_robot_cost_ore = parts[18].parse::<i32>().unwrap();
        let obsidian_robot_cost_clay = parts[21].parse::<i32>().unwrap();
        let geode_robot_cost_ore = parts[27].parse::<i32>().unwrap();
        let geode_robot_cost_obsidian = parts[30].parse::<i32>().unwrap();
    
        let max_ore_use_per_turn = ore_robot_cost.max( clay_robot_cost.max( obsidian_robot_cost_ore.max( geode_robot_cost_ore)));
        blueprints.push( Blueprint{ ore_robot_cost, clay_robot_cost, obsidian_robot_cost_ore,  
            obsidian_robot_cost_clay, geode_robot_cost_ore, geode_robot_cost_obsidian, max_ore_use_per_turn } );
    }


    // Part 1
    let mut quality_level = 0;
    for (idx, blueprint )in blueprints.iter().enumerate() {
        // Start with enough ore to build an ore-robot ( the problem states you already have it but this way the search starts after the 'first' one is built) 
        // (for the same reason we add an extra turn)
        let initial_state =  State{ ore: blueprint.ore_robot_cost, clay: 0, obsidian: 0, geodes: 0, ore_robots: 0, clay_robots: 0, obsidian_robots: 0, geode_robots: 0,
            construction: Construction::OreRobot };
        let most_geods = find_most_geodes( &blueprint, 24+1, initial_state );
        quality_level += (idx as i32 + 1)*most_geods;
        println!("{}, {}, {}", idx+1, quality_level, most_geods );
    } 
    println!( "Total quality level is {}",quality_level );

    // Part 2
    let mut result = 1;
    for (idx, blueprint )in blueprints[0..3].iter().enumerate() {
        // Start with enough ore to build an ore-robot ( the problem states you already have it but this way the search starts after the 'first' one is built) 
        // (for the same reason we add an extra turn)
        let initial_state =  State{ ore: blueprint.ore_robot_cost, clay: 0, obsidian: 0, geodes: 0, ore_robots: 0, clay_robots: 0, obsidian_robots: 0, geode_robots: 0,
            construction: Construction::OreRobot };
        let most_geods = find_most_geodes( &blueprint, 32+1, initial_state );
        result *= most_geods;
        println!("{}, {}, {}", idx+1, result, most_geods );
    } 
    println!( "Part 2 result is {}",result );
}
