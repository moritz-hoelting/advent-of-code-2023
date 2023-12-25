use pathfinding::prelude::bfs;
use std::collections::{HashMap, HashSet};

/// Puzzle consists only of one part today.
fn main() {
    println!("{}", part1(include_str!("./input.txt")));
}

fn part1(input: &str) -> usize {
    let mut graph = graph_from_input(input);

    for i in 1..graph.len() {
        let paths = (0..3)
            .map(|_| {
                let path = bfs(
                    &0,
                    |node| graph[node].iter().copied(),
                    |&node| node == i as u16,
                )
                .unwrap();

                // remove chosen edges
                path.windows(2).for_each(|e| {
                    graph.get_mut(&e[0]).unwrap().remove(&e[1]);
                    graph.get_mut(&e[1]).unwrap().remove(&e[0]);
                });

                path
            })
            .collect::<Vec<_>>();

        // check if graph is still connected
        match bfs(
            &0,
            |node| graph[node].iter().copied(),
            |&node| node == i as u16,
        ) {
            Some(_) => (),
            None => {
                let size_a = connected_count(&graph);
                let size_b = graph.len() - size_a;

                return size_a * size_b;
            }
        }

        // restore edges
        paths.into_iter().for_each(|path| {
            path.windows(2).for_each(|e| {
                graph.get_mut(&e[0]).unwrap().insert(e[1]);
                graph.get_mut(&e[1]).unwrap().insert(e[0]);
            });
        });
    }

    0
}

type Graph = HashMap<u16, HashSet<u16>>;

fn graph_from_input(input: &str) -> Graph {
    let mut node_ids = HashMap::new();
    let mut nodes = HashMap::new();
    for line in input.lines() {
        let (start, ends) = line.split_once(": ").unwrap();
        for end in ends.split(' ') {
            let id = node_ids.len() as u16;
            node_ids.entry(end).or_insert(id);
        }
        let id = node_ids.len() as u16;
        node_ids.entry(start).or_insert(id);

        let start = node_ids[&start];
        let ends = ends.split(' ').map(|e| node_ids[&e]);
        for end in ends.clone() {
            nodes.entry(end).or_insert_with(HashSet::new).insert(start);
        }
        nodes.entry(start).or_insert_with(HashSet::new).extend(ends);
    }

    nodes
}

fn connected_count(nodes: &Graph) -> usize {
    let mut visited = HashSet::new();
    let mut queue = Vec::new();
    queue.push(*nodes.keys().next().unwrap());
    while let Some(node) = queue.pop() {
        if !visited.insert(node) {
            continue;
        }
        for &neighbor in nodes[&node].iter() {
            queue.push(neighbor);
        }
    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(indoc!(
                "
                jqt: rhn xhk nvd
                rsh: frs pzl lsr
                xhk: hfx
                cmg: qnr nvd lhk bvb
                rhn: xhk bvb hfx
                bvb: xhk hfx
                pzl: lsr hfx nvd
                qnr: nvd
                ntq: jqt hfx bvb xhk
                nvd: lhk
                lsr: lhk
                rzs: qnr cmg lsr rsh
                frs: qnr lhk lsr
                "
            )),
            54
        );
    }
}
