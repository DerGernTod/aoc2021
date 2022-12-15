use std::rc::Rc;

use super::state::State;

pub struct StateByCheapest(pub Rc<State>);

impl PartialEq for StateByCheapest {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for StateByCheapest {}

impl PartialOrd for StateByCheapest {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for StateByCheapest {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.0.visited == other.0.visited {
            if self.0.cheapest_path == other.0.cheapest_path {
                self.0.pods.cmp(&other.0.pods)
            } else {
                self.0.cheapest_path.cmp(&other.0.cheapest_path)
            }
        } else {
            self.0.visited.cmp(&other.0.visited)
        }
    }
}