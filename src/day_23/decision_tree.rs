
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
}

struct LinkNode {
    block: Block,
    next: Vec<LinkNode>
}

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
    fn block_loops(&self, block: Block) -> bool {
        for blocks in &self.blocked_moves {
            if blocks.blocked_by == block.blocked_id {
                return true;
            }
        }
        false
    }
    fn is_deadlocked(&self) -> bool {
        let blocks = &self.blocked_moves;
        for block in blocks {
            let mut cur_block = block;
        }
        for (id, blocks) in blocks_by_id {
            for block in blocks {

            }
        }
        false
    }

    pub fn evaluate(&mut self, grid: &mut Grid) -> Option<u32> {
        println!("{} evaluating pod {}", " ".repeat(self.depth), self.pod_id);
        if self.is_deadlocked() {
            println!("{} pod {} is deadlocked! locks: {:?}", " ".repeat(self.depth), self.pod_id, self.blocked_moves);
            return None;
        }
        if let Some(transforms) = &self.transforms {
            grid.exec_transformations(transforms, false);
            grid.pods.get_mut(&self.pod_id).unwrap().walked_count += 1;
            println!("applied transforms to: \n{:?}\n", grid);
        }
        if self.pod_id < 8 && grid.is_pod_in_goal(self.pod_id) {
            // println!("{} pod {} reached end with cost of {}", " ".repeat(self.depth + 1), self.pod_id, self.self_cost);
            return Some(self.self_cost);
        }
        let self_pod = grid.pods.get(&self.pod_id).unwrap();
        if self.transforms.is_none() {
            let goals = self_pod.calc_goals();
            let mut occupants = HashSet::new();
            for (_, goal) in goals {
                match grid.follow_to_goal_dec(self_pod.id, goal) {
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
            if !grid.is_pod_in_goal(self_pod.id) && self_pod.walked_count == 1 && self_pod.is_in_goal_area() {
                grid.exec_transformations(self.transforms.as_ref().unwrap(), true);
                grid.pods.get_mut(&self.pod_id).unwrap().walked_count -= 1;
                println!("reversed transforms to: \n{:?}\n", grid);
                return None;
            }
            // otherwise add all remaining pods to my children
            for pod_id in grid.find_incomplete_pods() {
                self.add_child(DecisionTree::new(pod_id, 0, None, self.blocked_moves.clone(), self.depth + 1), false);
            }
        }
        let mut results: Vec<u32> = self.children
            .iter_mut()
            .filter_map(|child_tree| child_tree.evaluate(grid)
                .and_then(|cost| 
                    if !grid.is_pod_in_goal(child_tree.pod_id) {
                        child_tree.evaluate(grid).map(|double_cost| cost + double_cost)
                    } else {
                        Some(cost)
                    }))
                .map(|cost| cost + self.self_cost)
            .collect();
        results.sort();
        let best_result = results.first().copied();
        if best_result.is_none() {
            println!("{} found no way for {}!", " ".repeat(self.depth), self.pod_id);
            if let Some(transforms) = &self.transforms {
                grid.exec_transformations(transforms, true);
                grid.pods.get_mut(&self.pod_id).unwrap().walked_count -= 1;
                println!("reversed transforms to: \n{:?}\n", grid);
            }
        }
        best_result
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
        // let blocks = DecisionTree::new_test_blocked(vec![Block::new(1, 2), Block::new(2, 1)]);
        // assert!(blocks.is_deadlocked());
        // let blocks = DecisionTree::new_test_blocked(vec![Block::new(1, 2), Block::new(2, 3), Block::new(3, 1)]);
        // assert!(blocks.is_deadlocked());
        // let blocks = DecisionTree::new_test_blocked(vec![Block::new(1, 2), Block::new(2, 3), Block::new(3, 4)]);
        // assert!(!blocks.is_deadlocked());
        // let blocks = DecisionTree::new_test_blocked(vec![Block::new(7, 3), Block::new(0, 1), Block::new(1, 2), Block::new(2, 1)]);
        // assert!(blocks.is_deadlocked());
        let mut blocks = DecisionTree::new_test_blocked(HashSet::from([
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