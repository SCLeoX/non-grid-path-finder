use noisy_float::prelude::*;

use bv::BitVec;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub trait AStarInput {
    fn neighbors(&self, node: usize) -> &[usize];
    fn distance(&self, from: usize, to: usize) -> N64;
    fn heuristic(&self, node: usize) -> N64;
    fn len(&self) -> usize;
    fn start(&self) -> usize;
    fn end(&self) -> usize;
}

#[derive(Eq, PartialEq, Clone, Copy)]
struct NodeCost {
    node: usize,
    f_score: N64,
}

impl Ord for NodeCost {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .f_score
            .cmp(&self.f_score)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for NodeCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn a_star<Input>(input: &Input) -> Option<Vec<usize>>
where
    Input: AStarInput,
{
    let mut open_queue = BinaryHeap::new();
    let mut closed_set = BitVec::<usize>::new_fill(false, input.len() as u64);
    let start = input.start();
    let end = input.end();
    open_queue.push(NodeCost {
        node: start,
        f_score: input.heuristic(start),
    });
    let mut came_from = vec![usize::MAX; input.len()];
    let mut g_score = vec![n64(f64::INFINITY); input.len()];
    g_score[start] = n64(0.);
    while let Some(NodeCost { node: current, .. }) = open_queue.pop() {
        if current == end {
            let mut path = vec![];
            let mut current = current;
            while current != usize::MAX {
                path.push(current);
                current = came_from[current];
            }
            path.reverse();
            return Some(path);
        }
        if closed_set[current as u64] {
            continue;
        }
        for &neighbor in input.neighbors(current) {
            let tentative_g_score = g_score[current] + input.distance(current, neighbor);
            if tentative_g_score < g_score[neighbor] {
                came_from[neighbor] = current;
                g_score[neighbor] = tentative_g_score;
                open_queue.push(NodeCost {
                    node: neighbor,
                    f_score: tentative_g_score + input.heuristic(neighbor),
                });
                closed_set.set(current as u64, true);
            }
        }
    }
    None
}
