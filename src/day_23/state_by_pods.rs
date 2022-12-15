use std::rc::Rc;

use super::state::State;

pub struct StateByPods(pub Rc<State>);

impl PartialEq for StateByPods {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for StateByPods {}

impl PartialOrd for StateByPods {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for StateByPods {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.pods.cmp(&other.0.pods)
    }
}