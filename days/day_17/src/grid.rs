use std::{ops::Index, collections::{HashMap, BTreeSet, HashSet}};

use crate::{direction::Direction, position::Position};

#[derive(Debug)]
pub struct Grid {
    matrix: Vec<Vec<u8>>,
    pub n: usize,
    pub m: usize,
}

impl Grid {
    pub fn new(matrix: Vec<Vec<u8>>) -> Self {
        let n = matrix.len();
        let m = matrix[0].len();
        Self { matrix, n, m }
    }

    pub fn next_in_direction(&self, from: Position, dir: Direction) -> Option<Position> {
        self.next_in_direction_n(from, dir, 1)
    }

    pub fn next_in_direction_n(&self, from: Position, dir: Direction, n: isize) -> Option<Position> {
        let (di, dj) = dir.delta();
        let i = from.i.checked_add_signed(di * n)?;
        let j = from.j.checked_add_signed(dj * n)?;
        if i >= self.n || j >= self.m {
            return None;
        }
        Some(Position::new(i, j))
    }

    pub fn minimize_loss(&self, from: Position, to: Position) -> u64 {
        use crate::node::{Node, NodeInfo};

        let mut queue: BTreeSet<Node> = BTreeSet::from(Node::from_pos(from));
        let mut result: HashMap<NodeInfo, Node> = HashMap::from(Node::from_pos(from).map(|node: Node| (node.info, node)));
        let mut visited: HashSet<NodeInfo> = HashSet::new();
        loop {
            let node = queue.pop_first().unwrap();
            if node.info.pos == to {
                return node.heat
            }
            if !visited.insert(node.info) {
                continue;
            }
            for node in node.connected_to(self).iter().flatten() {
                if let Some(current_node) = result.get(&node.info) {
                    let minimal_node = if current_node.heat > node.heat {node} else {current_node};
                    queue.replace(*current_node);
                    result.insert(node.info, *minimal_node);
                } else {
                    queue.insert(*node);
                    result.insert(node.info, *node);
                };
            }
        }
    }

    pub fn minimize_loss_2(&self, from: Position, to: Position) -> u64 {
        use crate::node_2::{Node, NodeInfo};
        
        let mut queue: BTreeSet<Node> = BTreeSet::from(Node::from_pos(from));
        let mut result: HashMap<NodeInfo, Node> = HashMap::from(Node::from_pos(from).map(|node: Node| (node.info, node)));
        let mut visited: HashSet<NodeInfo> = HashSet::new();
        loop {
            let node = queue.pop_first().unwrap();
            if node.info.pos == to {
                return node.heat
            }
            if !visited.insert(node.info) {
                continue;
            }
            for node in node.connected_to(self).iter().flatten() {
                if let Some(current_node) = result.get(&node.info) {
                    let minimal_node = if current_node.heat > node.heat {node} else {current_node};
                    queue.replace(*current_node);
                    result.insert(node.info, *minimal_node);
                } else {
                    queue.insert(*node);
                    result.insert(node.info, *node);
                };
            }
        }
    }
}

impl Index<Position> for Grid {
    type Output = u8;

    fn index(&self, index: Position) -> &Self::Output {
        &self.matrix[index]
    }
}
