use std::{fmt, collections::HashSet};

const HALLWAY_GOALS: [(u32, u32); 7] = [(1, 1), (2, 1), (4, 1), (6, 1), (8, 1), (10, 1), (11, 1)];

pub struct PodFactory {
    next_id: usize
}
impl PodFactory {
    pub fn new() -> PodFactory {
        PodFactory { next_id: 0 }
    }

    pub fn new_pod(&mut self, kind: PodKind, location: (u32, u32)) -> Pod {
        let pod_id = self.next_id;
        self.next_id += 1;
        Pod::new(kind, location, pod_id)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub enum PodKind {
    Amber = 1,
    Bronze = 10,
    Copper = 100,
    Desert = 1000
}
impl PodKind {
    pub fn val(&self) -> u32 {
        *self as u32
    }
    pub fn goal_col(&self) -> u32 {
        match *self {
            Self::Amber => 3,
            Self::Bronze => 5,
            Self::Copper => 7,
            Self::Desert => 9
        }
    }
}

pub fn calc_next_step((x, y): (u32, u32), (goal_x, goal_y): (u32, u32)) -> Option<(u32, u32)> {
    if x == goal_x {
        match y.cmp(&goal_y) {
            std::cmp::Ordering::Less => Some((x, y + 1)),
            std::cmp::Ordering::Equal => None,
            std::cmp::Ordering::Greater => Some((x, y - 1)),
        }
    } else if y > 1 {
        Some((x, y - 1))
    } else if x < goal_x {
        Some((x + 1, y))
    } else if x > goal_x {
        Some((x - 1, y))
    } else {
        None
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Pod {
    pub id: usize,
    pub location: (u32, u32),
    pub kind: PodKind,
    pub walked_count: u32,
    pub blocked_goals: HashSet<(u32, (u32, u32))>
}

impl Pod {
    fn new(kind: PodKind, location: (u32, u32), id: usize) -> Pod {
        Pod {
            id,
            kind,
            location,
            walked_count: 0,
            blocked_goals: HashSet::new()
        }
    }
    
    pub fn is_in_goal_area(&self) -> bool {
        let (x, y) = self.location;
        x == self.kind.goal_col() && y > 1
    }

    fn calc_costs_for_goal(&self, goal: (u32, u32)) -> u32 {
        let mut cur_pos = self.location;
        let mut cost = 0;
        if goal.1 == 1 {
            // hallway goal
            while let Some(next) = calc_next_step(cur_pos, goal) {
                cur_pos = next;
                cost += self.kind.val();
            }
        }
        if cur_pos.0 == self.kind.goal_col() && cur_pos.1 > 1 && cur_pos.1 != goal.1 {
            return self.kind.val();
        }
        while let Some(next) = calc_next_step(cur_pos, (self.kind.goal_col(), 2)) {
            cur_pos = next;
            cost += self.kind.val();
        }
        cost
    }

    pub fn calc_goals(&self) -> Vec<(u32, (u32, u32))> {
        let mut goals = vec![];
        let (x, y) = self.location;
        let goal_col = self.kind.goal_col();
        if x == goal_col && y == 3 {
            return vec![(0, self.location)];
        }
        if self.walked_count > 0 && x == goal_col {
            return vec![];
        }
        let upper_goal = (goal_col, 2);
        let upper_cost = self.calc_costs_for_goal(upper_goal);
        let lower_goal = (goal_col, 3);
        let lower_cost = self.calc_costs_for_goal(lower_goal);
        if x != goal_col || self.walked_count == 0 {
            goals.push((upper_cost, upper_goal));
            goals.push((lower_cost, lower_goal));
        }
        if y != 1 {
            goals.append(&mut HALLWAY_GOALS.into_iter().map(|goal|(self.calc_costs_for_goal(goal), goal)).collect());
        } 
 
        goals.sort_by_key(|(cost, _)| *cost);
        goals
    }

}

impl fmt::Display for Pod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:}", match self.kind {
            PodKind::Amber => "A",
            PodKind::Bronze => "B",
            PodKind::Copper => "C",
            PodKind::Desert => "D"
        })
    }
}
impl Ord for Pod {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.kind.cmp(&self.kind)
    }
}
impl PartialOrd for Pod {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.kind.partial_cmp(&self.kind)
    }
}


#[cfg(test)]
mod tests {
    use super::{calc_next_step, Pod, PodKind};

    #[test]
    fn test_calc_cost_for_goal() {
        let pod = Pod::new(PodKind::Bronze, (3, 2), 0);
        assert_eq!(pod.calc_costs_for_goal((1, 1)), 80);
        assert_eq!(pod.calc_costs_for_goal((5, 3)), 40);
    }
    #[test]
    fn test_calc_goals() {
        let pod = Pod::new(PodKind::Bronze, (3, 2), 0);
        let goals = pod.calc_goals();
        assert_eq!(goals.len(), 9);
        assert_eq!(goals[8].0, 40);
        assert_eq!(goals[7].0, 40);
        assert_eq!(goals[6].0, 40);
        assert_eq!(goals[5].0, 60);
        assert_eq!(goals[4].0, 60);
        assert_eq!(goals[3].0, 80);
        assert_eq!(goals[2].0, 100);
        assert_eq!(goals[1].0, 140);
        assert_eq!(goals[0].0, 160);
        let pod = Pod::new(PodKind::Bronze, (5, 2), 0);
        let goals = pod.calc_goals();
        assert_eq!(goals.len(), 9);
        let mut pod = Pod::new(PodKind::Bronze, (5, 2), 0);
        pod.walked_count = 1;
        let goals = pod.calc_goals();
        assert_eq!(goals.len(), 0);
        let pod = Pod::new(PodKind::Bronze, (3, 1), 0);
        let goals = pod.calc_goals();
        assert_eq!(goals.len(), 2);
    }
    #[test]
    fn test_calc_next_step_goal() {
        assert_eq!(calc_next_step((3, 2), (5, 3)), Some((3, 1)));
        assert_eq!(calc_next_step((3, 1), (5, 3)), Some((4, 1)));
        assert_eq!(calc_next_step((4, 1), (5, 3)), Some((5, 1)));
        assert_eq!(calc_next_step((5, 1), (5, 3)), Some((5, 2)));
        assert_eq!(calc_next_step((5, 2), (5, 3)), Some((5, 3)));
        assert_eq!(calc_next_step((5, 3), (5, 3)), None);
    }
    #[test]
    fn test_calc_next_step_hallway() {
        assert_eq!(calc_next_step((3, 2), (11, 1)), Some((3, 1)));
        assert_eq!(calc_next_step((3, 1), (11, 1)), Some((4, 1)));
        assert_eq!(calc_next_step((4, 1), (11, 1)), Some((5, 1)));
        assert_eq!(calc_next_step((5, 1), (11, 1)), Some((6, 1)));
        assert_eq!(calc_next_step((6, 1), (11, 1)), Some((7, 1)));
        assert_eq!(calc_next_step((7, 1), (11, 1)), Some((8, 1)));
        assert_eq!(calc_next_step((8, 1), (11, 1)), Some((9, 1)));
        assert_eq!(calc_next_step((9, 1), (11, 1)), Some((10, 1)));
        assert_eq!(calc_next_step((10, 1), (11, 1)), Some((11, 1)));
        assert_eq!(calc_next_step((11, 1), (11, 1)), None);
    }
    #[test]
    fn test_calc_next_step_hallway_to_goal() {
        assert_eq!(calc_next_step((11, 1), (5, 3)), Some((10, 1)));
        assert_eq!(calc_next_step((10, 1), (5, 3)), Some((9, 1)));
        assert_eq!(calc_next_step((9, 1), (5, 3)), Some((8, 1)));
        assert_eq!(calc_next_step((8, 1), (5, 3)), Some((7, 1)));
        assert_eq!(calc_next_step((7, 1), (5, 3)), Some((6, 1)));
        assert_eq!(calc_next_step((6, 1), (5, 3)), Some((5, 1)));
        assert_eq!(calc_next_step((5, 1), (5, 3)), Some((5, 2)));
        assert_eq!(calc_next_step((5, 2), (5, 3)), Some((5, 3)));
        assert_eq!(calc_next_step((5, 3), (5, 3)), None);
    }
}