
#[derive(PartialEq, Eq, Debug)]
pub struct MoveCommand {
    options: Vec<Vec<(u32, u32)>>,
    cur_opt_index: Option<usize>,
    cur_exec_index: usize
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

impl MoveCommand {
    pub fn new(start: (u32, u32), goals: Vec<(u32, u32)>) -> MoveCommand {
        let mut options = vec![];
        for goal in goals {
            let mut cur_step = start;
            let mut steps = vec![];
            while let Some(next_step) = calc_next_step(cur_step, goal) {
                cur_step = next_step;
                steps.push(next_step);
            }
            options.push(steps);
        }
        options.sort_by_key(|steps_a| steps_a.len());
        MoveCommand {
            options,
            cur_opt_index: None,
            cur_exec_index: 0
        }
    }
    pub fn start_next_option(&mut self) -> Option<()> {
        let new_index = self.cur_opt_index
            .map(|ind| ind + 1)
            .unwrap_or(0);
        if new_index >= self.options.len() {
            None
        } else {
            self.cur_opt_index = Some(new_index);
            self.cur_exec_index = 0;
            Some(())
        }
    }
    pub fn step(&mut self) -> Option<(u32, u32)> {
        let opt_index = self.cur_opt_index?;
        let cur_opt = &self.options[opt_index];
        if self.cur_exec_index < cur_opt.len() {
            let location = cur_opt[self.cur_exec_index];
            self.cur_exec_index += 1;
            Some(location)
        } else {
            None
        }
    }
    pub fn get_cur_opt_step_count(&self) -> usize {
        self.options[self.cur_opt_index.unwrap()].len()
    }
}

#[cfg(test)]
mod tests {
    use super::{calc_next_step, MoveCommand};

    #[test]
    fn test_new_option_order() {
        let mc = MoveCommand::new((3, 2), vec![(2, 1), (11, 1), (5, 3), (5, 2)]);
        assert_eq!(mc.options.len(), 4);
        assert_eq!(mc.options[0], vec![(3, 1), (2, 1)]);
        assert_eq!(mc.options[1].len(), 4);
        assert_eq!(mc.options[2].len(), 5);
        assert_eq!(mc.options[3].len(), 9);
    }
    #[test]
    fn test_step() {
        let mut mc = MoveCommand::new((3, 2), vec![(2, 1), (11, 1), (5, 3), (5, 2)]);
        assert_eq!(mc.step(), None);

        mc.start_next_option();
        assert_eq!(mc.step(), Some((3, 1)));
        assert_eq!(mc.step(), Some((2, 1)));
        assert_eq!(mc.step(), None);
        
        mc.start_next_option();
        assert_eq!(mc.step(), Some((3, 1)));
        assert_eq!(mc.step(), Some((4, 1)));
    }
    #[test]
    fn test_start_next_option() {
        let mut mc = MoveCommand::new((3, 2), vec![(2, 1), (11, 1), (5, 3), (5, 2)]);
        assert_eq!(mc.cur_opt_index, None);
        assert_eq!(mc.start_next_option(), Some(()));
        assert_eq!(mc.cur_opt_index, Some(0));
        assert_eq!(mc.start_next_option(), Some(()));
        assert_eq!(mc.cur_opt_index, Some(1));
        assert_eq!(mc.start_next_option(), Some(()));
        assert_eq!(mc.cur_opt_index, Some(2));
        assert_eq!(mc.start_next_option(), Some(()));
        assert_eq!(mc.cur_opt_index, Some(3));
        assert_eq!(mc.start_next_option(), None);
        assert_eq!(mc.cur_opt_index, Some(3));
        assert_eq!(mc.start_next_option(), None);
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