use std::{fs, collections::{BTreeMap, BTreeSet}, rc::Rc};

use self::{state::State, pod::{PodKind, Pod}, coords::Coords, state_by_pods::StateByPods, state_by_cheapest::StateByCheapest};

mod coords;
mod pod;
mod state;
mod state_by_cheapest;
mod state_by_pods;

// this is basically brute force + djikstra:
// - create lists of remaining and all states, put starting state in both lists
// - look at the current state. if it's the target, return its costs. if not
// - generate all neighbors = all states this state can transition to
// - calculate each neighbors cost and add current state's cost
// - insert or update all those neighbor states into lists of states, ordered by cost (update cost if smaller): remove state from remaining states if visited
// - mark current state as visited
// - remove current state from remaining states, pick next remaining state

// part 1 is rather fast, part 2 can take half an hour... 
// could probably be improved a lot by using a* instead of djikstra,
// e.g. by using num pods in goal as heuristics

pub fn day_23() {
    let state = parse_into_init_state("./input/day_23.txt", false);
    let cost = find_path_to_goal(state, generate_target_state(false), false);
    println!("cheapest way: {cost}");
    let state = parse_into_init_state("./input/day_23.txt", true);
    let cost = find_path_to_goal(state, generate_target_state(true), true);
    println!("cheapest way unfolded: {cost}");
}

fn update_new_states(cur_state: &Rc<State>, all_states: &BTreeSet<StateByPods>, use_size_four: bool) -> Vec<Rc<State>> {
    cur_state
        .calc_possible_neighbors(use_size_four)
        .into_iter()
        .flat_map(|state| {
            if let Some(existing_state) = all_states.get(&StateByPods(Rc::clone(&state))) {
                if existing_state.0.cheapest_path > state.cheapest_path {
                    Some(state)
                } else {
                    None
                }
            } else {
                Some(state)
            }
        })
        .collect()
}

fn find_path_to_goal(init_state: State, target_state: State, use_size_four: bool) -> usize {
    let mut remaining_states: BTreeSet<StateByCheapest> = BTreeSet::new();
    let mut all_states: BTreeSet<StateByPods> = BTreeSet::new();
    let init_rc = Rc::new(init_state);
    let target_rc = Rc::new(target_state);
    remaining_states.insert(StateByCheapest(Rc::clone(&init_rc)));
    all_states.insert(StateByPods(Rc::clone(&init_rc)));
    while let Some(mut cur_state) = remaining_states.pop_first() {
        
        let states = update_new_states(&cur_state.0, &all_states, use_size_four);
        for mut created_state in states {
            if let Some(existing_state) = all_states.get(&StateByPods(Rc::clone(&created_state))) {
                remaining_states.remove(&StateByCheapest(Rc::clone(&existing_state.0)));
                if created_state.visited != existing_state.0.visited {
                    let mut cloned_state = Rc::make_mut(&mut created_state).to_owned();
                    cloned_state.visited = existing_state.0.visited;
                    created_state = Rc::new(cloned_state);
                }
                if !existing_state.0.visited {
                    remaining_states.insert(StateByCheapest(Rc::clone(&created_state)));
                }
                all_states.insert(StateByPods(created_state));
            } else {
                all_states.insert(StateByPods(Rc::clone(&created_state)));
                remaining_states.insert(StateByCheapest(Rc::clone(&created_state)));
            }
        }
        all_states.remove(&StateByPods(Rc::clone(&cur_state.0)));
        if cur_state.0 == target_rc {
            println!("Found a solution with {} cost", cur_state.0.cheapest_path);
            return cur_state.0.cheapest_path;
        }
        let mut clone = Rc::make_mut(&mut cur_state.0).to_owned();
        clone.visited = true;
        all_states.insert(StateByPods(Rc::new(clone)));
    }
    println!("Found a total of {} states", all_states.len());
    panic!("No path to solution found!");
}

fn parse_into_init_state(path: &str, use_size_four: bool) -> State {
    let input = fs::read_to_string(path).unwrap();
    let mut pods: BTreeMap<Coords, Pod> = input.lines().skip(2).take(2).enumerate()
        .flat_map(|(y, line)| {
            let shifted_y = if use_size_four && y > 0 { y + 2 } else { y };
            line
            .chars()
            .enumerate()
            .flat_map(move |(x, ch)| PodKind::from(ch).map(|kind| ((x, shifted_y + 2), Pod::new((x, shifted_y + 2), kind, 0, use_size_four))))
         })
        .collect();
    
        if use_size_four {
            pods.insert((3, 3), Pod::new((3, 3), PodKind::Desert, 0, true));
            pods.insert((5, 3), Pod::new((5, 3), PodKind::Copper, 0, true));
            pods.insert((7, 3), Pod::new((7, 3), PodKind::Bronze, 0, true));
            pods.insert((9, 3), Pod::new((9, 3), PodKind::Amber, 0, true));
            pods.insert((3, 4), Pod::new((3, 4), PodKind::Desert, 0, true));
            pods.insert((5, 4), Pod::new((5, 4), PodKind::Bronze, 0, true));
            pods.insert((7, 4), Pod::new((7, 4), PodKind::Amber, 0, true));
            pods.insert((9, 4), Pod::new((9, 4), PodKind::Copper, 0, true));
        }
    State::new(pods, 0)
}

fn generate_target_state(use_size_four: bool) -> State {
    let lower_goal_row = if use_size_four { 5 } else { 3 };
    let mut pods = BTreeMap::from([
        ((3, 2), Pod::new((3, 2), PodKind::Amber, 0, use_size_four)),
        ((3, lower_goal_row), Pod::new((3, lower_goal_row), PodKind::Amber, 0, use_size_four)),
        ((5, 2), Pod::new((5, 2), PodKind::Bronze, 0, use_size_four)),
        ((5, lower_goal_row), Pod::new((5, lower_goal_row), PodKind::Bronze, 0, use_size_four)),
        ((7, 2), Pod::new((7, 2), PodKind::Copper, 0, use_size_four)),
        ((7, lower_goal_row), Pod::new((7, lower_goal_row), PodKind::Copper, 0, use_size_four)),
        ((9, 2), Pod::new((9, 2), PodKind::Desert, 0, use_size_four)),
        ((9, lower_goal_row), Pod::new((9, lower_goal_row), PodKind::Desert, 0, use_size_four)),
    ]);
    if use_size_four {
        pods.insert((3, 3), Pod::new((3, 3), PodKind::Amber, 0, true));
        pods.insert((3, 4), Pod::new((3, 4), PodKind::Amber, 0, true));
        pods.insert((5, 3), Pod::new((5, 3), PodKind::Bronze, 0, true));
        pods.insert((5, 4), Pod::new((5, 4), PodKind::Bronze, 0, true));
        pods.insert((7, 3), Pod::new((7, 3), PodKind::Copper, 0, true));
        pods.insert((7, 4), Pod::new((7, 4), PodKind::Copper, 0, true));
        pods.insert((9, 3), Pod::new((9, 3), PodKind::Desert, 0, true));
        pods.insert((9, 4), Pod::new((9, 4), PodKind::Desert, 0, true));
    }
    State::new(pods, 0)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::day_23::find_path_to_goal;

    use super::{parse_into_init_state, pod::{PodKind, Pod}, generate_target_state};

    #[test]
    fn test_part_1() {
        let state = parse_into_init_state("./input/day_23.test.txt", false);
        let cost = find_path_to_goal(state, generate_target_state(false), false);
        assert_eq!(cost, 12521);
    }
    #[test]
    fn test_part_2() {
        let state = parse_into_init_state("./input/day_23.test.txt", true);
        let cost = find_path_to_goal(state, generate_target_state(true), true);
        assert_eq!(cost, 44169);
    }

    #[test]
    fn test_parse() {
        let state = parse_into_init_state("./input/day_23.test.txt", false);
        let pods_mock = BTreeMap::from([
            ((3, 2), Pod::new((3, 2), PodKind::Bronze, 0, false)),
            ((5, 2), Pod::new((5, 2), PodKind::Copper, 0, false)),
            ((7, 2), Pod::new((7, 2), PodKind::Bronze, 0, false)),
            ((9, 2), Pod::new((9, 2), PodKind::Desert, 0, false)),
            ((3, 3), Pod::new((3, 3), PodKind::Amber, 0, false)),
            ((5, 3), Pod::new((5, 3), PodKind::Desert, 0, false)),
            ((7, 3), Pod::new((7, 3), PodKind::Copper, 0, false)),
            ((9, 3), Pod::new((9, 3), PodKind::Amber, 0, false)),
        ]);
        let pods_mock_different_order = BTreeMap::from([
            ((5, 2), Pod::new((5, 2), PodKind::Copper, 0, false)),
            ((3, 3), Pod::new((3, 3), PodKind::Amber, 5, false)),
            ((7, 2), Pod::new((7, 2), PodKind::Bronze, 0, false)),
            ((9, 3), Pod::new((9, 3), PodKind::Amber, 9, false)),
            ((3, 2), Pod::new((3, 2), PodKind::Bronze, 0, false)),
            ((5, 3), Pod::new((5, 3), PodKind::Desert, 3, false)),
            ((7, 3), Pod::new((7, 3), PodKind::Copper, 2, false)),
            ((9, 2), Pod::new((9, 2), PodKind::Desert, 99, false)),
        ]);
        assert_eq!(pods_mock, pods_mock_different_order);
        assert_eq!(pods_mock, state.pods);
        for (coords, pod) in pods_mock {
            assert_eq!(state.pods.get(&coords), Some(&pod), "Pod not included: {pod:?}");
        }

    }

    
}