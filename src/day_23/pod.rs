use std::fmt;

use super::{cell::CellType, move_command::MoveCommand};

const HALLWAY_GOALS: [(u32, u32); 7] = [(1, 1), (2, 1), (4, 1), (6, 1), (8, 1), (10, 1), (11, 1)];

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

#[derive(PartialEq, Eq, Debug)]
pub struct Pod {
    pub location: (u32, u32),
    pub commands: Vec<MoveCommand>,
    pub kind: PodKind,
    pub is_pushed_by: Option<(u32, u32)>
}

impl Pod {
    pub fn new(kind: PodKind, location: (u32, u32)) -> Pod {
        Pod {
            kind,
            location,
            commands: vec![],
            is_pushed_by: None
        }
    }
    
    pub fn get_total_move_costs(&self) -> u32 {
        self.commands
            .iter()
            .map(|cmd| cmd.get_cur_opt_step_count() as u32)
            .sum::<u32>()
            * self.kind.val()
    }

    pub fn calc_goals(&self) -> Vec<(u32, u32)> {
        let mut goals = vec![];
        let (x, y) = self.location;
        // todo: calc all possible targets
        if y != 1 {
            goals = Vec::from(HALLWAY_GOALS);
        }
        if x != self.kind.goal_col() {
            goals.push((self.kind.goal_col(), 2));
            goals.push((self.kind.goal_col(), 3));
        }
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