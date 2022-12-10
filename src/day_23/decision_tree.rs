
use std::collections::HashSet;

use super::{transformation::Transform, pod::PodKind, grid::{Grid, PodMoveResultDec}};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Block {
    blocked_id: usize,
    blocked_by: usize
}
impl Block {
    pub fn new(blocked_id: usize, blocked_by: usize) -> Block {
        Block {
            blocked_by,
            blocked_id
        }
    }
    fn is_deadlocked(&self, blocks_in_loop: &[Block], all_blocks: &[Block], block_offset: usize) -> bool {
        let mut others_clone = all_blocks.to_owned();
        others_clone.rotate_left(block_offset + 1);
        let mut new_loop_blocks = blocks_in_loop.to_owned();
        new_loop_blocks.push(*self);
        for (offset, other) in others_clone.iter().enumerate() {
            if other.blocked_id == self.blocked_by
                && (blocks_in_loop.contains(other) 
                    || other.is_deadlocked(&new_loop_blocks, &others_clone, offset)) {
                return true;
            }
        }
        false
    }
}

type IterationStep = Vec<(usize, Vec<Transform>)>;
pub struct DecisionTree {
    self_cost: u32,
    depth: usize,
    blocked_moves: HashSet<Block>,
    preferred_follows: Vec<usize>,
    pub pod_id: usize,
    pub transforms: Option<Vec<Transform>>,
    pub children: Vec<DecisionTree>
}

enum GoalResults {
    Hit(Vec<usize>),
    Pass(DecisionTree),
    ReachedEnd
}

impl DecisionTree {
    pub fn new(pod_id: usize, self_cost: u32, transforms: Option<Vec<Transform>>, blocked_moves: HashSet<Block>, depth: usize, preferred_follows: Vec<usize>) -> DecisionTree {
        DecisionTree {
            pod_id,
            depth,
            self_cost,
            blocked_moves,
            transforms,
            preferred_follows,
            children: vec![]
        }
    }
    pub fn from_pods(pods: Vec<(PodKind, usize)>) -> Vec<DecisionTree> {
        pods.iter()
            .map(|(kind, pod_id)| DecisionTree::new(*pod_id, kind.val(), None, HashSet::new(), 1, vec![]))
            .collect()
    }
    fn add_child(&mut self, child: DecisionTree) {
        self.children.push(child);
    }

    fn is_deadlocked(&self) -> bool {
        let blocks = &self.blocked_moves;
        let blocks_vec: Vec<Block> = blocks.clone().into_iter().collect();
        for block in blocks {
            if block.is_deadlocked(&[], &blocks_vec, 0) {
                return true;
            }
        }
        false
    }

    pub fn evaluate(&mut self, grid: &mut Grid) -> Option<u32> {
        // println!("{} evaluating pod {}", " ".repeat(self.depth), self.pod_id);
        if self.is_deadlocked() || self.depth > 500 {
            // println!("{} pod {} is deadlocked! locks: {:?}", " ".repeat(self.depth), self.pod_id, self.blocked_moves);
            return None;
        }
        if let Some(transforms) = &self.transforms {
            grid.exec_transformations(transforms, self.pod_id);
            // if !transforms.is_empty() {
            //     println!("{grid}\n");
            // }
        }
        if self.transforms.is_none() {
            let goals = grid.calc_goals_for_pod(self.pod_id, &self.preferred_follows);
            let occupants: Vec<GoalResults> = goals
                .into_iter()
                .map(|(_, goal)| match grid.follow_to_goal_dec(self.pod_id, goal) {
                    PodMoveResultDec::Hit(cur_occupants) => { GoalResults::Hit(cur_occupants) },
                    PodMoveResultDec::Pass(cost, transforms) => {
                        GoalResults::Pass(DecisionTree::new(self.pod_id, cost, Some(transforms), self.blocked_moves.clone(), self.depth + 1, self.preferred_follows.clone()))
                    },
                    PodMoveResultDec::ReachedEnd(cost, transforms) => {
                        self.add_child(DecisionTree::new(self.pod_id, cost, Some(transforms), self.blocked_moves.clone(), self.depth + 1, self.preferred_follows.clone()));
                        GoalResults::ReachedEnd
                    },
                })
                .collect();
            let mut stacks_to_skip = HashSet::new();
            for goal_result in occupants.into_iter() {
                match goal_result {
                    GoalResults::Hit(occupant_stack) => {
                        
                        if stacks_to_skip.contains(&occupant_stack) {
                            continue;
                        }
                        let block = Block::new(self.pod_id, *occupant_stack.last().unwrap());
                        let mut blocked_clone = self.blocked_moves.clone();
                        blocked_clone.insert(block);
                        let mut stack_clone = occupant_stack.clone();
                        let occupant = stack_clone.pop().unwrap();
                        stack_clone.insert(0, self.pod_id);
                        self.add_child(DecisionTree::new(occupant, 0, None, blocked_clone, self.depth + 1, stack_clone));
                        stacks_to_skip.insert(occupant_stack);
                    },
                    GoalResults::Pass(tree) => self.add_child(tree),
                    GoalResults::ReachedEnd => (),
                }
                
            }
        } else {
            // println!("{} moving pod {} to {:?}", " ".repeat(self.depth), self.pod_id, self.transforms.as_ref().unwrap().last().unwrap());

            // if i already have a transform, it means i finished moving.
            // in case i already moved once, and i'm in goal area but not in end, 
            // there's no way for me. 
            let self_pod = grid.pods.get(&self.pod_id).unwrap();
            if !grid.is_pod_in_goal(self_pod.id) && self_pod.walked_count == 1 && self_pod.is_in_goal_area() {
                return None;
            }
            if let Some(id) = self.preferred_follows.last() {
                if *id == self.pod_id {
                    self.preferred_follows.pop();
                }
            }
            // otherwise add free pods first,
            // then pods that triggered me and if there's none,
            // all remaining pods to my children, prefer prev occupants
            let free_pods = grid.find_pods_that_can_move_to_goal();
            let incomplete_pods: Vec<usize> = grid
                .find_incomplete_pods()
                .into_iter()
                .filter(|pod_id| !self.preferred_follows.contains(pod_id))
                .filter(|pod_id| !free_pods.iter().any(|(free_id, _)| pod_id == free_id))
                .collect();
            for (pod_id, transforms) in free_pods {
                let free_cost = transforms.iter().map(|Transform(cost, _, _)| cost).sum();
                self.add_child(DecisionTree::new(pod_id, free_cost, Some(transforms), self.blocked_moves.clone(), self.depth + 1, self.preferred_follows.clone()));
            }
                
            let mut preferred_pods = self.preferred_follows.clone();
            if let Some(preferred) = preferred_pods.pop() {
                self.add_child(DecisionTree::new(preferred, 0, None, self.blocked_moves.clone(), self.depth + 1, preferred_pods));
            } else {
                for pod_id in incomplete_pods {
                    self.add_child(DecisionTree::new(pod_id, 0, None, self.blocked_moves.clone(), self.depth + 1, self.preferred_follows.clone()));
                }
            }
        }
        let mut results: Option<(u32, IterationStep)> = self.children
            .iter_mut()
            .find_map(|child_tree| {
                let start_offset = grid.get_iteration_len();
                if let Some(cost) = child_tree.evaluate(grid) {
                    // if we're already in goal area but not in goal, there's no way for us
                    if !grid.is_pod_in_goal(child_tree.pod_id) && grid.pods.get(&child_tree.pod_id).unwrap().is_in_goal_area() {
                        grid.reverse_iterations(grid.get_iteration_len() - start_offset);
                        None
                    } else {
                        let reversed = grid.reverse_iterations(grid.get_iteration_len() - start_offset);
                        Some((cost, reversed))
                    }
                } else {
                    grid.reverse_iterations(grid.get_iteration_len() - start_offset);
                    None
                }
            });
        // results.sort_by_key(|(cost, _)| *cost);
        if let Some((cost, iterations)) = results {
            grid.apply_iterations(iterations);
            Some(cost + self.self_cost)
        } else if self.pod_id < 8 && grid.is_pod_in_goal(self.pod_id) && grid.all_pods_in_goal() {
            // println!("{} pod {} reached end with cost of {}", " ".repeat(self.depth + 1), self.pod_id, self.self_cost);
            Some(self.self_cost)
        } else if self.transforms.is_some() {
            grid.reverse_iterations(1);
            // println!("{grid}\n");
            None
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    use crate::day_23::decision_tree::Block;

    use super::DecisionTree;
    impl DecisionTree {
        pub fn new_test_blocked(blocked_moves: HashSet<Block>) -> DecisionTree {
            DecisionTree::new(0, 0, None, blocked_moves, 0, vec![])
        }
    }
    #[test]
    fn test_is_deadlock() {
        let blocks = DecisionTree::new_test_blocked(HashSet::from([Block::new(1, 2), Block::new(2, 1)]));
        assert!(blocks.is_deadlocked());
        let blocks = DecisionTree::new_test_blocked(HashSet::from([Block::new(1, 2), Block::new(2, 3), Block::new(3, 1)]));
        assert!(blocks.is_deadlocked());
        let blocks = DecisionTree::new_test_blocked(HashSet::from([Block::new(1, 2), Block::new(2, 3), Block::new(3, 4)]));
        assert!(!blocks.is_deadlocked());
        let blocks = DecisionTree::new_test_blocked(HashSet::from([Block::new(7, 3), Block::new(0, 1), Block::new(1, 2), Block::new(2, 1)]));
        assert!(blocks.is_deadlocked());
        let blocks = DecisionTree::new_test_blocked(HashSet::from([
            Block::new(5, 1),
            Block::new(1, 3),
            Block::new(3, 7),
            Block::new(7, 0),
            Block::new(5, 1),
            Block::new(1, 3),
            Block::new(3, 7),
            Block::new(5, 1),
            Block::new(1, 2),
            Block::new(2, 0),
            Block::new(0, 1),
        ]));
        assert!(blocks.is_deadlocked());
    }

    #[test]
    fn test_block_is_deadlocked() {
        let mut block_list = vec![Block::new(0, 1), Block::new(1, 2), Block::new(2, 3)];
        assert!(!block_list[0].is_deadlocked(&[], &block_list, 0));
        block_list.rotate_left(1);
        assert!(!block_list[0].is_deadlocked(&[], &block_list, 0));
        block_list.rotate_left(1);
        assert!(!block_list[0].is_deadlocked(&[], &block_list, 0));

        let mut block_list = vec![Block::new(0, 1), Block::new(1, 2), Block::new(2, 0)];
        assert!(block_list[0].is_deadlocked(&[], &block_list, 0));
        block_list.rotate_left(1);
        assert!(block_list[0].is_deadlocked(&[], &block_list, 0));
        block_list.rotate_left(1);
        assert!(block_list[0].is_deadlocked(&[], &block_list, 0));

        let block_list = vec![
            Block::new(5, 1),
            Block::new(1, 3),
            Block::new(3, 7),
            Block::new(7, 0),
            Block::new(1, 2),
            Block::new(2, 0),
            Block::new(0, 1),
        ];
        assert!(block_list[0].is_deadlocked(&[], &block_list, 0));

        let block_list = vec![
            Block::new(5, 1),
            Block::new(2, 5),
            Block::new(1, 2),
            Block::new(3, 7),
            Block::new(5, 2),
            Block::new(5, 0),
            Block::new(7, 0),
            Block::new(2, 3),
        ];
        assert!(block_list[0].is_deadlocked(&[], &block_list, 0));
        assert!(block_list[1].is_deadlocked(&[], &block_list, 1));
        assert!(block_list[2].is_deadlocked(&[], &block_list, 2));
        assert!(!block_list[3].is_deadlocked(&[], &block_list, 3));
        assert!(block_list[4].is_deadlocked(&[], &block_list, 4));
        assert!(!block_list[5].is_deadlocked(&[], &block_list, 5));
        assert!(!block_list[6].is_deadlocked(&[], &block_list, 6));
        assert!(!block_list[7].is_deadlocked(&[], &block_list, 7));

        let block_list = vec![
            Block::new(7, 5),
            Block::new(1, 5),
            Block::new(7, 3),
            Block::new(2, 5)
        ];
        assert!(!block_list[0].is_deadlocked(&[], &block_list, 0));
        assert!(!block_list[1].is_deadlocked(&[], &block_list, 1));
        assert!(!block_list[2].is_deadlocked(&[], &block_list, 2));
        assert!(!block_list[3].is_deadlocked(&[], &block_list, 3));

    }

}