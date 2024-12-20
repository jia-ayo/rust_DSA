use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};
type Graph<V, E> = BTreeMap<V, BTreeMap<V, E>>;

fn add_edge<V: Ord + Copy, E: Ord + Copy>(graph: &mut Graph<V, E>, v1: V, v2: V, wt: E) {
    graph.entry(v1).or_insert_with(BTreeMap::new).insert(v2, wt);
    graph.entry(v2).or_insert_with(BTreeMap::new).insert(v1, wt);
}

pub fn prim<V: Ord + Copy, E: Ord + Copy>(graph: &mut Graph<V, E>, start: V) -> Graph<V, E> {
    let mut mst = Graph::new();
    let mut visited = BinaryHeap::new();
    mst.insert(start, BTreeMap::new());

    for (v, wt) in &graph[&start] {
        visited.push(Reverse((*wt, v, start)));
    }

    while let Some(Reverse((wt, vt, prev))) = visited.pop(){
        if mst.contains_key(vt){
            continue;
        }
        add_edge(&mut mst, prev, *vt, wt);
        for (v, wt) in &graph[vt] {
            if !mst.contains_key(v){
                visited.push(Reverse((*wt, v, *vt)));
            }
        }
    }
    mst
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_prims(){
        let mut graph = BTreeMap::new();
        add_edge(&mut graph, 0, 1, 3);
        add_edge(&mut graph, 0, 2, 4);
        add_edge(&mut graph, 1, 2, 1);
        add_edge(&mut graph, 1, 4, 6);
        add_edge(&mut graph, 2, 4, 3);
        add_edge(&mut graph, 2, 3, 5);
        add_edge(&mut graph, 3, 4, 2);

        let mut ans=  BTreeMap::new();
        add_edge(&mut ans, 0, 1, 3);
        add_edge(&mut ans, 1, 2, 1);
        add_edge(&mut ans, 2, 4, 3);
        add_edge(&mut ans, 3, 4, 2);

        assert_eq!(prim(&mut graph, 0), ans);
    }
}

/*use disjoint_sets::UnionFind;

type Node = usize;
type Weight = usize;
struct Edge {
    dest: Node,
    weight: Weight
}

type Graph = Vec<Vec<Edge>>;

fn edges_by_weight(graph: &Graph ) -> Vec<(Node, Node, Weight)> {
    let mut edges = vec![];

    for (src, dest)  in graph.iter().enumerate() {
        for edge in dest {
            edges.push((src, edge.dest, edge.weight));
        }
    }
    edges.sort_by_key(|&(_, _, weight)| weight);
    edges
}

fn mst(graph: &Graph) -> Vec<(Node, Node)> {
    let mut result= vec![];
    let mut union_find = UnionFind::new(graph.len());
    for (src, dest, _) in edges_by_weight(graph) {
        if !union_find.equiv(src, dest){
            union_find.union(src, dest);
            result.push((src, dest));
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_mst(){
        let graph = vec![
            //Node 0
            vec![
                Edge{ dest: 1, weight: 3},
                Edge{ dest: 3, weight: 6},
                Edge{ dest: 5, weight: 1},
            ],
            //Node 1
            vec![
                Edge{ dest: 3, weight: 5},
                Edge{ dest: 5, weight: 4},
                Edge{ dest: 2, weight: 1},
            ],
            //Node 2
            vec![
                Edge{ dest: 3, weight: 2},
                Edge{ dest: 4, weight: 3},
            ],
            //Node 3
            vec![
                Edge{ dest: 4, weight: 7},
            ],
            //Node 4
            vec![
                Edge{ dest: 5, weight: 2},
            ],
            //Node 5
            vec![
            ],

        ];
        assert_eq!(vec![(0,5), (1,2),  (2,3), (4,5), (0,1)], mst(&graph))
    }
}*/
