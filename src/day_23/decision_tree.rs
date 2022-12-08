
use std::collections::{HashSet, HashMap};

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
    pub pod_id: usize,
    pub transforms: Option<Vec<Transform>>,
    pub children: Vec<DecisionTree>
}

impl DecisionTree {
    pub fn new(pod_id: usize, self_cost: u32, transforms: Option<Vec<Transform>>, blocked_moves: HashSet<Block>, depth: usize) -> DecisionTree {
        DecisionTree {
            pod_id,
            depth,
            self_cost,
            blocked_moves,
            transforms,
            children: vec![]
        }
    }
    pub fn from_pods(pods: Vec<(PodKind, usize)>) -> Vec<DecisionTree> {
        pods.iter()
            .map(|(kind, pod_id)| DecisionTree::new(*pod_id, kind.val(), None, HashSet::new(), 1))
            .rev()
            .collect()
    }
    pub fn add_child(&mut self, child: DecisionTree, reverse: bool) {
        if reverse {
            match self.children.binary_search_by(|val| child.self_cost.cmp(&val.self_cost)) {
                Ok(pos) => self.children.insert(pos, child),
                Err(pos) => self.children.insert(pos, child),
            }
        } else {
            match self.children.binary_search_by(|val| val.self_cost.cmp(&child.self_cost)) {
                Ok(pos) => self.children.insert(pos, child),
                Err(pos) => self.children.insert(pos, child),
            }
        }
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
        if self.is_deadlocked() {
            // println!("{} pod {} is deadlocked! locks: {:?}", " ".repeat(self.depth), self.pod_id, self.blocked_moves);
            return None;
        }
        if let Some(transforms) = &self.transforms {
            grid.exec_transformations(transforms, self.pod_id);
            if !transforms.is_empty() {
                println!("{grid}\n");
            }
        }
        if self.pod_id < 8 && grid.is_pod_in_goal(self.pod_id) {
            // println!("{} pod {} reached end with cost of {}", " ".repeat(self.depth + 1), self.pod_id, self.self_cost);
            return Some(self.self_cost);
        }
        if self.transforms.is_none() {
            let goals = grid.calc_goals_for_pod(self.pod_id);
            let mut occupants = HashSet::new();
            for (_, goal) in goals {
                match grid.follow_to_goal_dec(self.pod_id, goal) {
                    PodMoveResultDec::Hit(occupant) => { occupants.insert(occupant); },
                    PodMoveResultDec::Pass(cost, transforms) 
                    | PodMoveResultDec::ReachedEnd(cost, transforms) => {
                        self.add_child(DecisionTree::new(self.pod_id, cost, Some(transforms), self.blocked_moves.clone(), self.depth + 1), false);
                    },
                };
            }
            for occupant in occupants {
                let block = Block::new(self.pod_id, occupant);
                let mut blocked_clone = self.blocked_moves.clone();
                blocked_clone.insert(block);
                self.add_child(DecisionTree::new(occupant, 0, None, blocked_clone, self.depth + 1), false);
            }
        } else {
            // if i already have a transform, it means i finished moving.
            // in case i already moved once, and i'm in goal area but not in end, 
            // there's no way for me. 
            let self_pod = grid.pods.get(&self.pod_id).unwrap();
            if !grid.is_pod_in_goal(self_pod.id) && self_pod.walked_count == 1 && self_pod.is_in_goal_area() {
                return None;
            }
            // otherwise add all remaining pods to my children
            for pod_id in grid.find_incomplete_pods() {
                self.add_child(DecisionTree::new(pod_id, 0, None, self.blocked_moves.clone(), self.depth + 1), false);
            }
        }
        let mut results: Vec<(u32, IterationStep)> = self.children
            .iter_mut()
            .filter_map(|child_tree| {
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
            }).collect();
        results.sort_by_key(|(cost, _)| *cost);
        if let Some((cost, iterations)) = results.into_iter().next() {
            grid.apply_iterations(iterations);
            Some(cost + self.self_cost)
        } else {
            if self.transforms.is_some() {
                grid.reverse_iterations(1);
                println!("{grid}\n");
            }
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
        pub fn new_test(cost: u32) -> DecisionTree {
            DecisionTree::new(0, cost, None, HashSet::new(), 0)
        }
        pub fn new_test_blocked(blocked_moves: HashSet<Block>) -> DecisionTree {
            DecisionTree::new(0, 0, None, blocked_moves, 0)
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
        let mut set = HashSet::new();
        set.insert(Block::new(5, 1));
        set.insert(Block::new(5, 1));
        assert_eq!(set.len(), 1);
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

    }

    #[test]
    fn test_add_child() {
        let mut tree = DecisionTree::new_test(1);
        let child_large = DecisionTree::new_test(5);
        let child_small = DecisionTree::new_test(1);
        tree.add_child(child_small, false);
        tree.add_child(child_large, false);
        assert_eq!(tree.children[0].self_cost, 1);
        assert_eq!(tree.children[1].self_cost, 5);

        let mut tree = DecisionTree::new_test(1);
        let child_large = DecisionTree::new_test(5);
        let child_small = DecisionTree::new_test(1);
        tree.add_child(child_large, false);
        tree.add_child(child_small, false);
        assert_eq!(tree.children[0].self_cost, 1);
        assert_eq!(tree.children[1].self_cost, 5);
        
        let mut tree = DecisionTree::new_test(1);
        let child_large = DecisionTree::new_test(5);
        let child_small = DecisionTree::new_test(1);
        tree.add_child(child_small, true);
        tree.add_child(child_large, true);
        assert_eq!(tree.children[0].self_cost, 5);
        assert_eq!(tree.children[1].self_cost, 1);

        let mut tree = DecisionTree::new_test(1);
        let child_large = DecisionTree::new_test(5);
        let child_small = DecisionTree::new_test(1);
        tree.add_child(child_large, true);
        tree.add_child(child_small, true);
        assert_eq!(tree.children[0].self_cost, 5);
        assert_eq!(tree.children[1].self_cost, 1);
    }
    
}