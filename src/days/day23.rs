use std::{convert::TryFrom, iter::FromIterator, sync::Arc};

use itertools::Itertools;

const TARGET_LOCATIONS: usize = 7;
// Forward costs from one location to another (to be fair it could just be one long vector)
const FORWARD_COSTS: [u32; 56] = [
    3, 2, 2, 4, 6, 8, 9, 5, 4, 2, 2, 4, 6, 7, 7, 6, 4, 2, 2, 4, 5, 9, 8, 6, 4, 2, 2, 3, 4, 3, 3, 5,
    7, 9, 10, 6, 5, 3, 3, 5, 7, 8, 8, 7, 5, 3, 3, 5, 6, 10, 9, 7, 5, 3, 3, 4,
];

/*
fn axis_range(input: &str) -> IResult<&str, (i32, i32)> {
    let (rem_str, (first_raw, second_raw)) = preceded(
        alt((tag("x="), tag("y="), tag("z="))),
        separated_pair(
            pair(opt(tag("-")), digit1),
            tag(".."),
            pair(opt(tag("-")), digit1),
        ),
    )(input)?;

    let first_value = format!("{}{}", first_raw.0.unwrap_or(""), first_raw.1);
    let second_value = format!("{}{}", second_raw.0.unwrap_or(""), second_raw.1);

    Ok((
        rem_str,
        (first_value.parse().unwrap(), second_value.parse().unwrap()),
    ))
}
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
enum AmphiType {
    Amber = 1,
    Bronze = 10,
    Copper = 100,
    Desert = 1000,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Amphipod {
    node: usize,
    race: AmphiType,
    back_in_slot: bool,
}

fn parse_input(input: &str) -> [Amphipod; 8] {
    let mut result: [Amphipod; 8] = [Amphipod {
        node: 255,
        race: AmphiType::Amber,
        back_in_slot: false,
    }; 8];
    for (row, line) in input.lines().skip(2).take(2).map(|l| l.trim()).enumerate() {
        let cleaned_string = line.trim_matches('#');

        println!("Cleaned string: {}", cleaned_string);

        for (col, char) in cleaned_string.chars().step_by(2).take(4).enumerate() {
            let race = match char {
                'A' => AmphiType::Amber,
                'B' => AmphiType::Bronze,
                'C' => AmphiType::Copper,
                'D' => AmphiType::Desert,
                _ => panic!("We got a strange character between the amphipods!"),
            };
            let flat_id = row * 4 + col;
            result[flat_id] = Amphipod {
                node: flat_id + TARGET_LOCATIONS,
                race,
                back_in_slot: false,
            };
        }
    }

    let arrived_amphis = check_arrived(&result);

    for id in arrived_amphis.into_iter() {
        result[id].back_in_slot = true;
    }

    result
}

fn check_arrived(amphis: &[Amphipod; 8]) -> Vec<usize> {
    let mut arrived_amphis: Vec<usize> = Vec::new();
    for (id, amphi) in amphis.iter().enumerate() {
        match amphi.race {
            AmphiType::Amber => {
                if amphi.node == TARGET_LOCATIONS + 4 {
                    arrived_amphis.push(id);
                }
            }
            AmphiType::Bronze => {
                if amphi.node == TARGET_LOCATIONS + 5 {
                    arrived_amphis.push(id);
                }
            }
            AmphiType::Copper => {
                if amphi.node == TARGET_LOCATIONS + 6 {
                    arrived_amphis.push(id);
                }
            }
            AmphiType::Desert => {
                if amphi.node == TARGET_LOCATIONS + 7 {
                    arrived_amphis.push(id);
                }
            }
        }
    }

    let mut second_slot: Vec<usize> = Vec::new();

    for id in arrived_amphis.iter() {
        let arrived = &amphis[*id];
        for (first_row_id, amphi) in amphis.iter().enumerate() {
            if amphi.race == arrived.race && amphi.node == arrived.node - 4 {
                second_slot.push(first_row_id);
            };
        }
    }

    arrived_amphis.extend(second_slot.into_iter());

    arrived_amphis
}

fn get_target_node(amphi: &Amphipod, already_arrived: &[usize]) -> usize {
    let deep_node: usize = TARGET_LOCATIONS
        + match amphi.race {
            AmphiType::Amber => 4,
            AmphiType::Bronze => 5,
            AmphiType::Copper => 6,
            AmphiType::Desert => 7,
        };

    if already_arrived.contains(&deep_node) {
        deep_node - 4
    } else {
        deep_node
    }
}

fn get_forward_cost(start_node: usize, target_node: usize) -> u32 {
    let flat_id = (start_node - TARGET_LOCATIONS) * TARGET_LOCATIONS + target_node;
    FORWARD_COSTS[flat_id]
}

fn get_backwards_cost(start_node: usize, target_node: usize) -> u32 {
    let flat_id = (target_node - TARGET_LOCATIONS) * TARGET_LOCATIONS + start_node;
    FORWARD_COSTS[flat_id]
}

fn print_state(amphis: &[Amphipod; 8]) {
    let mut hallway: String = String::from("#...........#");
    let mut first_nodes: String = String::from("###.#.#.#.###");
    let mut second_nodes: String = String::from("  #.#.#.#.#");

    print!("Placing pods in their spaces...");
    for amphi in amphis {
        let amphi_char = match amphi.race {
            AmphiType::Amber => "A",
            AmphiType::Bronze => "B",
            AmphiType::Copper => "C",
            AmphiType::Desert => "D",
        };
        print!(" {} [{}];", amphi_char, amphi.node);
        if amphi.node < TARGET_LOCATIONS {
            let string_loc = if amphi.node < 2 {
                amphi.node + 1
            } else if amphi.node < 5 {
                amphi.node * 2
            } else {
                amphi.node + 5
            };
            hallway.replace_range(string_loc..string_loc + 1, amphi_char);
        } else if amphi.node < TARGET_LOCATIONS + 4 {
            let string_loc = (amphi.node - TARGET_LOCATIONS) * 2;
            first_nodes.replace_range(string_loc + 3..string_loc + 4, amphi_char);
        } else {
            let string_loc = (amphi.node - TARGET_LOCATIONS - 4) * 2;
            second_nodes.replace_range(string_loc + 3..string_loc + 4, amphi_char);
        }
    }
    println!();

    println!("#############");
    println!("{}", hallway);
    println!("{}", first_nodes);
    println!("{}", second_nodes);
    println!("  #########");
    println!();
}

fn find_cost(
    amphis: [Amphipod; 8],
    current_cost: u32,
    current_minimum: Option<u32>,
) -> Option<u32> {
    let mut cost: Option<u32> = None;

    let occupied_hallway_nodes = amphis
        .iter()
        .filter(|amphi| amphi.node < 7)
        .map(|amphi| amphi.node)
        .collect_vec();

    let arrived_amphis = check_arrived(&amphis);
    if arrived_amphis.len() == amphis.len() {
        return Some(current_cost);
    }

    /*
    println!("New iteration! ==============================================");
    println!("Current state:");
    println!();
    print_state(&amphis);
    */

    for (amphi_id, amphi) in amphis
        .iter()
        .enumerate()
        .filter(|(id, _)| !arrived_amphis.contains(id))
    {
        if amphi.node < TARGET_LOCATIONS {
            let (hallway_target, mut target_node) = match amphi.race {
                AmphiType::Amber => (1, TARGET_LOCATIONS + 4),
                AmphiType::Bronze => (2, TARGET_LOCATIONS + 5),
                AmphiType::Copper => (3, TARGET_LOCATIONS + 6),
                AmphiType::Desert => (4, TARGET_LOCATIONS + 7),
            };
            if arrived_amphis
                .iter()
                .filter(|&id| amphis[*id].race == amphi.race)
                .count()
                > 0
            {
                target_node -= 4;
            }

            let target_node_available =
                amphis.iter().filter(|amp| amp.node == target_node).count() == 0;
            let path_to_target_clear = occupied_hallway_nodes
                .iter()
                .filter(|&node| {
                    if amphi.node == *node {
                        false
                    } else if amphi.node > hallway_target {
                        *node > hallway_target
                    } else {
                        *node <= hallway_target
                    }
                })
                .count()
                == 0;

            if target_node_available && path_to_target_clear {
                let mut new_state = amphis;
                new_state[amphi_id].node = target_node;
                new_state[amphi_id].back_in_slot = true;
                let new_cost = current_cost
                    + (get_backwards_cost(amphi.node, target_node) * amphi.race as u32);
                if let Some(curr) = current_minimum {
                    if curr < new_cost {
                        return None;
                    }
                };
                if let Some(branch_cost) = find_cost(new_state, new_cost, cost) {
                    cost = Some(
                        cost.map_or(branch_cost, |previous_cost| previous_cost.min(branch_cost)),
                    );
                }
            }
        } else {
            let start_node = amphi.node;
            let hallway_target: usize = (0..4)
                .filter_map(|node| {
                    if start_node == TARGET_LOCATIONS + node
                        || start_node == TARGET_LOCATIONS + 4 + node
                    {
                        Some(node + 1)
                    } else {
                        None
                    }
                })
                .next()
                .unwrap();
            if occupied_hallway_nodes.is_empty() {
                for target in 0usize..7 {
                    let mut new_state = amphis;
                    new_state[amphi_id].node = target;
                    let new_cost =
                        current_cost + (get_forward_cost(amphi.node, target) * amphi.race as u32);
                    if let Some(curr) = current_minimum {
                        if curr < new_cost {
                            return None;
                        }
                    };
                    if let Some(branch_cost) = find_cost(new_state, new_cost, cost) {
                        cost =
                            Some(cost.map_or(branch_cost, |previous_cost| {
                                previous_cost.min(branch_cost)
                            }));
                    }
                }
            } else {
                for (target, occupied) in
                    (0usize..7).cartesian_product(occupied_hallway_nodes.iter())
                {
                    let path_available = if target <= hallway_target {
                        *occupied > hallway_target
                    } else {
                        *occupied <= hallway_target
                    };

                    if path_available {
                        let mut new_state = amphis;
                        new_state[amphi_id].node = target;
                        let new_cost = current_cost
                            + (get_forward_cost(amphi.node, target) * amphi.race as u32);
                        if let Some(curr) = current_minimum {
                            if curr < new_cost {
                                return None;
                            }
                        };
                        if let Some(branch_cost) = find_cost(new_state, new_cost, cost) {
                            cost = Some(cost.map_or(branch_cost, |previous_cost| {
                                previous_cost.min(branch_cost)
                            }));
                        }
                    }
                }
            }
        }
    }
    cost
}

#[cfg(test)]
mod tests {

    use hashbrown::HashMap;
    use itertools::Itertools;

    use super::*;

    const forward_network_A: [(u8, u8, u8); 7] = [
        (1, 0, 1),
        (7, 1, 2),
        (7, 2, 2),
        (2, 3, 2),
        (3, 4, 2),
        (4, 5, 2),
        (5, 6, 1),
    ];

    const forward_network_B: [(u8, u8, u8); 7] = [
        (1, 0, 1),
        (8, 2, 2),
        (8, 3, 2),
        (2, 1, 2),
        (3, 4, 2),
        (4, 5, 2),
        (5, 6, 1),
    ];

    const forward_network_C: [(u8, u8, u8); 7] = [
        (1, 0, 1),
        (9, 3, 2),
        (9, 4, 2),
        (2, 1, 2),
        (3, 2, 2),
        (4, 5, 2),
        (5, 6, 1),
    ];

    const forward_network_D: [(u8, u8, u8); 7] = [
        (1, 0, 1),
        (10, 4, 2),
        (10, 5, 2),
        (2, 1, 2),
        (3, 2, 2),
        (4, 3, 2),
        (5, 6, 1),
    ];

    fn explore_network(network: &[(u8, u8, u8); 7], start_node: u8) -> HashMap<(u8, u8), u8> {
        let mut costs: HashMap<(u8, u8), u8> = HashMap::new();
        let mut exploration: Vec<(u8, u8)> = network
            .iter()
            .filter_map(|(start, end, cost)| {
                if *start == start_node {
                    Some((*end, *cost))
                } else {
                    None
                }
            })
            .collect_vec();

        while let Some((next_node, next_cost)) = exploration.pop() {
            costs.insert((start_node, next_node), next_cost);
            costs.insert((start_node + 4, next_node), next_cost + 1);
            exploration.extend(network.iter().filter_map(|(start, end, cost)| {
                if *start == next_node {
                    Some((*end, *cost + next_cost))
                } else {
                    None
                }
            }));
        }

        costs
    }

    fn network_gen() {
        let mut total_forward_map = explore_network(&forward_network_A, 7);
        total_forward_map.extend(explore_network(&forward_network_B, 8));
        total_forward_map.extend(explore_network(&forward_network_C, 9));
        total_forward_map.extend(explore_network(&forward_network_D, 10));

        let total_forward_network = total_forward_map
            .into_iter()
            .sorted_by(|left, right| left.0.cmp(&right.0))
            .map(|(_, cost)| cost)
            .collect_vec();

        println!("Length: {}", total_forward_network.len());
        println!("{:?}", total_forward_network);
    }

    #[test]
    fn parse() {
        let input_str = "#############
        #...........#
        ###B#C#B#D###
          #A#D#C#A#
          #########";

        let amphis = parse_input(input_str);

        let ref_amphis = [
            Amphipod {
                node: 7,
                race: AmphiType::Bronze,
                back_in_slot: false,
            },
            Amphipod {
                node: 8,
                race: AmphiType::Copper,
                back_in_slot: false,
            },
            Amphipod {
                node: 9,
                race: AmphiType::Bronze,
                back_in_slot: false,
            },
            Amphipod {
                node: 10,
                race: AmphiType::Desert,
                back_in_slot: false,
            },
            Amphipod {
                node: 11,
                race: AmphiType::Amber,
                back_in_slot: true,
            },
            Amphipod {
                node: 12,
                race: AmphiType::Desert,
                back_in_slot: false,
            },
            Amphipod {
                node: 13,
                race: AmphiType::Copper,
                back_in_slot: true,
            },
            Amphipod {
                node: 14,
                race: AmphiType::Amber,
                back_in_slot: false,
            },
        ];

        assert_eq!(amphis, ref_amphis);
    }

    #[test]
    fn simple_run() {
        let input_str = "#############
        #...........#
        ###A#C#B#D###
          #A#B#C#D#
          #########";

        let amphis = parse_input(input_str);

        let run_cost = find_cost(amphis, 0, None);

        assert_eq!(run_cost, Some(460));
    }

    #[test]
    fn less_simple_run() {
        let input_str = "#############
        #...........#
        ###D#C#B#A###
          #A#B#C#D#
          #########";

        let amphis = parse_input(input_str);

        let run_cost = find_cost(amphis, 0, None);

        assert_eq!(run_cost, Some(8470));
    }

    #[test]
    fn full_run() {
        let input_str = "#############
        #...........#
        ###B#C#B#D###
          #A#D#C#A#
          #########";

        let amphis = parse_input(input_str);

        let run_cost = find_cost(amphis, 0, None);

        assert_eq!(run_cost, Some(12521));
    }
}
