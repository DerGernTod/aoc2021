use std::{fs, collections::{BTreeMap, BTreeSet}, rc::Rc};

use self::{state::State, pod::{PodKind, Pod}, coords::Coords, state_by_pods::StateByPods, state_by_cheapest::StateByCheapest};

mod coords;
mod pod;
mod state;
mod state_by_cheapest;
mod state_by_pods;

pub fn day_23() {
    let state = parse_into_init_state("./input/day_23.txt");
    let cost = find_path_to_goal(state, generate_target_state());
    println!("cheapest way: {cost}");
}

fn update_new_states(cur_state: &Rc<State>, all_states: &BTreeSet<StateByPods>) -> Vec<Rc<State>> {
    cur_state
        .calc_possible_neighbors()
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

fn find_path_to_goal(init_state: State, target_state: State) -> usize {
    let mut remaining_states: BTreeSet<StateByCheapest> = BTreeSet::new();
    let mut all_states: BTreeSet<StateByPods> = BTreeSet::new();
    let init_rc = Rc::new(init_state);
    let target_rc = Rc::new(target_state);
    remaining_states.insert(StateByCheapest(Rc::clone(&init_rc)));
    all_states.insert(StateByPods(Rc::clone(&init_rc)));
    while let Some(mut cur_state) = remaining_states.pop_first() {
        let states = update_new_states(&cur_state.0, &all_states);
        for mut created_state in states {
            if let Some(existing_state) = all_states.get(&StateByPods(Rc::clone(&created_state))) {
                remaining_states.remove(&StateByCheapest(Rc::clone(&existing_state.0)));
                let mut cloned_state = Rc::make_mut(&mut created_state).to_owned();
                cloned_state.visited = existing_state.0.visited;
                let cloned_rc = Rc::new(cloned_state);
                if !existing_state.0.visited {
                    remaining_states.insert(StateByCheapest(Rc::clone(&cloned_rc)));
                }
                all_states.insert(StateByPods(cloned_rc));
            } else {
                all_states.insert(StateByPods(Rc::clone(&created_state)));
                remaining_states.insert(StateByCheapest(Rc::clone(&created_state)));
            }
        }
        
        all_states.remove(&StateByPods(Rc::clone(&cur_state.0)));
        if cur_state.0 == target_rc {
            println!("Found a solution with {} cost", cur_state.0.cheapest_path);
        }
        let mut clone = Rc::make_mut(&mut cur_state.0).to_owned();
        clone.visited = true;
        all_states.insert(StateByPods(Rc::new(clone)));
    }
    println!("Found a total of {} states", all_states.len());
    all_states.get(&StateByPods(target_rc)).unwrap().0.cheapest_path
}

fn parse_into_init_state(path: &str) -> State {
    let input = fs::read_to_string(path).unwrap();
    let pods: BTreeMap<Coords, Pod> = input.lines().skip(2).take(2).enumerate()
        .flat_map(|(y, line)| line
            .chars()
            .enumerate()
            .flat_map(move |(x, ch)| PodKind::from(ch).map(|kind| ((x, y + 2), Pod::new((x, y + 2), kind, 0)))
            ))
        .collect();
    State::new(pods, 0)
}

fn generate_target_state() -> State {
    State::new(BTreeMap::from([
        ((3, 2), Pod::new((3, 2), PodKind::Amber, 0)),
        ((3, 3), Pod::new((3, 3), PodKind::Amber, 0)),
        ((5, 2), Pod::new((5, 2), PodKind::Bronze, 0)),
        ((5, 3), Pod::new((5, 3), PodKind::Bronze, 0)),
        ((7, 2), Pod::new((7, 2), PodKind::Copper, 0)),
        ((7, 3), Pod::new((7, 3), PodKind::Copper, 0)),
        ((9, 2), Pod::new((9, 2), PodKind::Desert, 0)),
        ((9, 3), Pod::new((9, 3), PodKind::Desert, 0)),
    ]), 0)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::day_23::find_path_to_goal;

    use super::{parse_into_init_state, pod::{PodKind, Pod}, generate_target_state};

    #[test]
    fn test_part_1() {
        let state = parse_into_init_state("./input/day_23.test.txt");
        let cost = find_path_to_goal(state, generate_target_state());
        assert_eq!(cost, 12521);
    }

    #[test]
    fn test_parse() {
        let state = parse_into_init_state("./input/day_23.test.txt");
        let pods_mock = BTreeMap::from([
            ((3, 2), Pod::new((3, 2), PodKind::Bronze, 0)),
            ((5, 2), Pod::new((5, 2), PodKind::Copper, 0)),
            ((7, 2), Pod::new((7, 2), PodKind::Bronze, 0)),
            ((9, 2), Pod::new((9, 2), PodKind::Desert, 0)),
            ((3, 3), Pod::new((3, 3), PodKind::Amber, 0)),
            ((5, 3), Pod::new((5, 3), PodKind::Desert, 0)),
            ((7, 3), Pod::new((7, 3), PodKind::Copper, 0)),
            ((9, 3), Pod::new((9, 3), PodKind::Amber, 0)),
        ]);
        let pods_mock_different_order = BTreeMap::from([
            ((5, 2), Pod::new((5, 2), PodKind::Copper, 0)),
            ((3, 3), Pod::new((3, 3), PodKind::Amber, 5)),
            ((7, 2), Pod::new((7, 2), PodKind::Bronze, 0)),
            ((9, 3), Pod::new((9, 3), PodKind::Amber, 9)),
            ((3, 2), Pod::new((3, 2), PodKind::Bronze, 0)),
            ((5, 3), Pod::new((5, 3), PodKind::Desert, 3)),
            ((7, 3), Pod::new((7, 3), PodKind::Copper, 2)),
            ((9, 2), Pod::new((9, 2), PodKind::Desert, 99)),
        ]);
        assert_eq!(pods_mock, pods_mock_different_order);
        assert_eq!(pods_mock, state.pods);
        for (coords, pod) in pods_mock {
            assert_eq!(state.pods.get(&coords), Some(&pod), "Pod not included: {pod:?}");
        }

    }

    
}