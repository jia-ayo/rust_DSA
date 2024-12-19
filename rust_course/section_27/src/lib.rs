// use std::collections::{HashSet, VecDeque};

// pub fn depth_first_serch(graph: &Graph, root: Vertex, objective: Vertex) -> Option<Vec<u8>> {
//     let mut visited: HashSet<Vertex> = HashSet::new();
//     let mut history: Vec<u8> = Vec::new();
//     let mut queue = VecDeque::new();

//     queue.push_back(root);

//     while let Some(current_vertex) = queue.pop_front() {
//         history.push(current_vertex.value());

//         if current_vertex == objective {
//             return Some(history);
//         }

//         for neighbor in current_vertex.neighbors(graph).into_iter().rev() {
//             if visited.insert(neighbor) {
//                 queue.push_front(neighbor)
//             }
//         }
//     }
//     None
// }

// pub fn breadth_first_search(graph: &Graph, root: Vertex, target: Vertex) -> Option<Vec<u8>> {
//     let mut visited: HashSet<Vertex> = HashSet::new();
//     let mut history: Vec<u8> = Vec::new();
//     let mut queue = VecDeque::new();

//     visited.insert(root);
//     queue.push_back(root);

//     while let Some(current_vartex) = queue.pop_front() {
//         history.push(current_vartex.value());

//         if current_vartex == target {
//             return Some(history);
//         }

//         for neighbor in current_vartex.neighbors(graph) {
//             if !visited.contains(&neighbor) {
//                 visited.insert(neighbor);
//                 queue.push_back(neighbor);
//             }
//         }
//     }
//     None
// }

// #[derive(Copy, Clone, PartialEq, Eq, Hash)]
// pub struct Vertex(u8);

// #[derive(Copy, Clone, PartialEq, Eq, Hash)]
// pub struct Edge(u8, u8);

// #[derive(Clone)]
// pub struct Graph {
//     #[allow(dead_code)]
//     vertices: Vec<Vertex>,
//     edges: Vec<Edge>,
// }

// impl Graph {
//     pub fn new(vertices: Vec<Vertex>, edges: Vec<Edge>) -> Self {
//         Graph { vertices, edges }
//     }
// }

// impl From<u8> for Vertex {
//     fn from(value: u8) -> Self {
//         Vertex(value)
//     }
// }

// impl Vertex {
//     pub fn value(&self) -> u8 {
//         self.0
//     }

//     pub fn neighbors(&self, graph: &Graph) -> VecDeque<Vertex> {
//         graph
//             .edges
//             .iter()
//             .filter(|e| e.0 == self.0)
//             .map(|e| e.1.into())
//             .collect()
//     }
// }

// impl From<(u8, u8)> for Edge {
//     fn from(value: (u8, u8)) -> Self {
//         Edge(value.0, value.1)
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_dfs() {
//         let vertices = vec![1, 2, 3, 4, 5, 6, 7];
//         let edges = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (3, 7)];

//         let root = 1;
//         let objective = 7;
//         let correct = vec![1, 2, 4, 5, 3, 6, 7];
//         let graph = Graph::new(
//             vertices.into_iter().map(|v| v.into()).collect(),
//             edges.into_iter().map(|e| e.into()).collect(),
//         );

//         assert_eq!(
//             depth_first_serch(&graph, root.into(), objective.into()),
//             Some(correct)
//         );
//     }

//     #[test]
//     fn fail_bfs() {
//         let vertices = vec![1, 2, 3, 4, 5, 6, 7];
//         let edges = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (3, 7)];

//         let root = 1;
//         let objective = 10;
//         let graph = Graph::new(
//             vertices.into_iter().map(|v| v.into()).collect(),
//             edges.into_iter().map(|e| e.into()).collect(),
//         );

//         assert_eq!(
//             breadth_first_search(&graph, root.into(), objective.into()),
//             None
//         );
//     }
//     #[test]
//     fn test_bfs() {
//         let vertices = vec![1, 2, 3, 4, 5, 6, 7];
//         let edges = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (3, 7)];

//         let root = 1;
//         let objective = 7;
//         let correct = vec![1, 2, 3, 4, 5, 6, 7];
//         let graph = Graph::new(
//             vertices.into_iter().map(|v| v.into()).collect(),
//             edges.into_iter().map(|e| e.into()).collect(),
//         );

//         assert_eq!(
//             breadth_first_search(&graph, root.into(), objective.into()),
//             Some(correct)
//         );
//     }
// }

//dijkstra algorithim

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::usize;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Edge {
    node: usize,
    cost: usize,
}

fn shortest_path(graph: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    let mut dist: Vec<_> = (0..graph.len()).map(|_| usize::MAX).collect();
    let mut visited = BinaryHeap::new();
    dist[start] = 0;
    visited.push(State {
        cost: 0,
        position: start,
    });
    while let Some(State { cost, position }) = visited.pop() {
        if position == goal {
            return Some(cost);
        }
        if cost > dist[position] {
            continue;
        }
        for edge in &graph[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };
            if next.cost < dist[next.position] {
                visited.push(next);
                dist[next.position] = next.cost;
            }
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_djkstra() {
        let graph = vec![
            // node 0
            vec![
                Edge { node: 1, cost: 6 },
                Edge { node: 2, cost: 4 },
                Edge { node: 3, cost: 1 },
            ],
            // node 1
            vec![Edge { node: 0, cost: 6 }, Edge { node: 2, cost: 3 }],
            //node 2
            vec![
                Edge {node: 0, cost: 4},
                Edge {node: 1, cost: 3},
                Edge {node: 3, cost: 1},
            ],
            //node 3
            vec![Edge { node: 0, cost: 1 }, Edge { node: 2, cost: 1 }],
        ];
        assert_eq!(shortest_path(&graph, 0, 1), Some(5));
    }
}
