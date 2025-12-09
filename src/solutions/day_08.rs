use std::collections::HashMap;

use itertools::Itertools;
use petgraph::prelude::*;

use crate::solution::Solution;

type Graph = UnGraph<JBox, ()>;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct JBox(i64, i64, i64);

impl JBox {
    fn parse(line: &str) -> Self {
        let (x, y, z) = line
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect_tuple()
            .unwrap();
        Self(x, y, z)
    }

    fn distance_to(&self, other: &Self) -> f64 {
        let sum = [self.0 - other.0, self.1 - other.1, self.2 - other.2]
            .iter()
            .map(|n| n * n)
            .sum::<i64>();
        (sum as f64).sqrt()
    }
}

#[derive(Debug)]
pub struct Day08 {
    boxes: Vec<JBox>,
}

impl Solution for Day08 {
    fn with_input(input: String) -> Self {
        let boxes = input.trim().lines().map(JBox::parse).collect_vec();
        Self { boxes }
    }

    fn part1(&self) -> String {
        let mut map = HashMap::with_capacity(self.boxes.len());
        let mut graph = Graph::with_capacity(self.boxes.len(), 1000);

        for jbox in &self.boxes {
            let node = graph.add_node(*jbox);
            map.insert(*jbox, node);
        }

        let mut distances = Vec::with_capacity(499500);
        distances.extend(
            self.boxes
                .iter()
                .tuple_combinations()
                .map(|(a, b)| (a.distance_to(b), a, b)),
        );

        distances.sort_by(|(dist_a, _, _), (dist_b, _, _)| dist_a.partial_cmp(dist_b).unwrap());

        for (_, a, b) in &distances[..1000] {
            let node_a = map.get(a).unwrap();
            let node_b = map.get(b).unwrap();

            graph.add_edge(*node_a, *node_b, ());
        }

        petgraph::algo::kosaraju_scc(&graph)
            .into_iter()
            .map(|scc| scc.len())
            .k_largest(3)
            .product::<usize>()
            .to_string()
    }
}
