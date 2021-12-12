//! # Day 12: Passage Pathing
//!
//! With your submarine's subterranean subsystems subsisting suboptimally, the only way you're getting out of this cave anytime soon is by finding a path yourself. Not just a path - the only way to know if you've found the best path is to find all of them.
//!
//! Fortunately, the sensors are still mostly working, and so you build a rough map of the remaining caves (your puzzle input). For example:
//!
//! start-A
//! start-b
//! A-c
//! A-b
//! b-d
//! A-end
//! b-end
//!
//! This is a list of how all of the caves are connected. You start in the cave named start, and your destination is the cave named end. An entry like b-d means that cave b is connected to cave d - that is, you can move between them.
//!
//! So, the above cave system looks roughly like this:
//!
//!     start
//!     /   \
//! c--A-----b--d
//!     \   /
//!      end
//!
//! Your goal is to find the number of distinct paths that start at start, end at end, and don't visit small caves more than once. There are two types of caves: big caves (written in uppercase, like A) and small caves (written in lowercase, like b). It would be a waste of time to visit any small cave more than once, but big caves are large enough that it might be worth visiting them multiple times. So, all paths you find should visit small caves at most once, and can visit big caves any number of times.
//!
//! Given these rules, there are 10 paths through this example cave system:
//!
//! start,A,b,A,c,A,end
//! start,A,b,A,end
//! start,A,b,end
//! start,A,c,A,b,A,end
//! start,A,c,A,b,end
//! start,A,c,A,end
//! start,A,end
//! start,b,A,c,A,end
//! start,b,A,end
//! start,b,end
//!
//! (Each line in the above list corresponds to a single path; the caves visited by that path are listed in the order they are visited and separated by commas.)
//!
//! Note that in this cave system, cave d is never visited by any path: to do so, cave b would need to be visited twice (once on the way to cave d and a second time when returning from cave d), and since cave b is small, this is not allowed.
//!
//! Here is a slightly larger example:
//!
//! dc-end
//! HN-start
//! start-kj
//! dc-start
//! dc-HN
//! LN-dc
//! HN-end
//! kj-sa
//! kj-HN
//! kj-dc
//!
//! The 19 paths through it are as follows:
//!
//! start,HN,dc,HN,end
//! start,HN,dc,HN,kj,HN,end
//! start,HN,dc,end
//! start,HN,dc,kj,HN,end
//! start,HN,end
//! start,HN,kj,HN,dc,HN,end
//! start,HN,kj,HN,dc,end
//! start,HN,kj,HN,end
//! start,HN,kj,dc,HN,end
//! start,HN,kj,dc,end
//! start,dc,HN,end
//! start,dc,HN,kj,HN,end
//! start,dc,end
//! start,dc,kj,HN,end
//! start,kj,HN,dc,HN,end
//! start,kj,HN,dc,end
//! start,kj,HN,end
//! start,kj,dc,HN,end
//! start,kj,dc,end
//!
//! Finally, this even larger example has 226 paths through it:
//!
//! fs-end
//! he-DX
//! fs-he
//! start-DX
//! pj-DX
//! end-zg
//! zg-sl
//! zg-pj
//! pj-he
//! RW-he
//! fs-DX
//! pj-RW
//! zg-RW
//! start-pj
//! he-WI
//! zg-he
//! pj-fs
//! start-RW
//!
//! How many paths through this cave system are there that visit small caves at most once?
//!
//! Your puzzle answer was 5457.
//!
//! ## Part Two
//!
//! After reviewing the available paths, you realize you might have time to visit a single small cave twice. Specifically, big caves can be visited any number of times, a single small cave can be visited at most twice, and the remaining small caves can be visited at most once. However, the caves named start and end can only be visited exactly once each: once you leave the start cave, you may not return to it, and once you reach the end cave, the path must end immediately.
//!
//! Now, the 36 possible paths through the first example above are:
//!
//! start,A,b,A,b,A,c,A,end
//! start,A,b,A,b,A,end
//! start,A,b,A,b,end
//! start,A,b,A,c,A,b,A,end
//! start,A,b,A,c,A,b,end
//! start,A,b,A,c,A,c,A,end
//! start,A,b,A,c,A,end
//! start,A,b,A,end
//! start,A,b,d,b,A,c,A,end
//! start,A,b,d,b,A,end
//! start,A,b,d,b,end
//! start,A,b,end
//! start,A,c,A,b,A,b,A,end
//! start,A,c,A,b,A,b,end
//! start,A,c,A,b,A,c,A,end
//! start,A,c,A,b,A,end
//! start,A,c,A,b,d,b,A,end
//! start,A,c,A,b,d,b,end
//! start,A,c,A,b,end
//! start,A,c,A,c,A,b,A,end
//! start,A,c,A,c,A,b,end
//! start,A,c,A,c,A,end
//! start,A,c,A,end
//! start,A,end
//! start,b,A,b,A,c,A,end
//! start,b,A,b,A,end
//! start,b,A,b,end
//! start,b,A,c,A,b,A,end
//! start,b,A,c,A,b,end
//! start,b,A,c,A,c,A,end
//! start,b,A,c,A,end
//! start,b,A,end
//! start,b,d,b,A,c,A,end
//! start,b,d,b,A,end
//! start,b,d,b,end
//! start,b,end
//!
//! The slightly larger example above now has 103 paths through it, and the even larger example now has 3509 paths through it.
//!
//! Given these new rules, how many paths through this cave system are there?
//!
//! Your puzzle answer was 128506.

use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};

use itertools::Itertools;

use crate::{day::Challenge, parse_input_str};

/// Day 12 implementation.
pub struct Day12;

fn save_graph_to_disk(graph: &Graph, output: &Path) {
    let mut dot_output = String::new();
    dot_output.push_str("graph G {\n");

    for (&node_id, node_links) in &graph.links {
        let node_name = &graph.nodes[node_id].name;
        for &target_id in node_links {
            let target_name = &graph.nodes[target_id].name;
            dot_output.push_str(&format!("  {} -- {}\n", node_name, target_name));
        }
    }

    dot_output.push('}');

    std::fs::write(output, dot_output).expect("Could not write DOT to disk.");
}

#[derive(Debug)]
struct Node {
    name: String,
    is_small_cave: bool,
}

impl Node {
    pub fn new(name: String) -> Self {
        let small = name.to_lowercase() == name;
        Self {
            name,
            is_small_cave: small,
        }
    }

    pub fn is_small_cave(&self) -> bool {
        self.is_small_cave
    }

    pub fn is_end(&self) -> bool {
        self.name == "end"
    }

    pub fn is_start(&self) -> bool {
        self.name == "start"
    }
}

#[derive(Debug)]
struct Graph {
    index: HashMap<String, usize>,
    nodes: Vec<Node>,
    links: HashMap<usize, HashSet<usize>>,
}

#[derive(Debug)]
struct Tree<T> {
    node: T,
    children: Vec<Tree<T>>,
}

impl<T: Clone + std::fmt::Debug> Tree<T> {
    pub fn new(elem: T) -> Self {
        Self {
            node: elem,
            children: Vec::new(),
        }
    }

    pub fn list_paths(&self) -> Vec<Vec<T>> {
        Self::list_paths_rec(self, vec![])
    }

    pub fn list_paths_rec(tree: &Self, mut path: Vec<T>) -> Vec<Vec<T>> {
        let mut out = vec![];
        path.push(tree.node.clone());

        if tree.children.is_empty() {
            out.push(path.clone());
        }

        for child in &tree.children {
            let new_path = path.clone();
            for p in Self::list_paths_rec(child, new_path) {
                out.push(p)
            }
        }

        out
    }
}

impl Graph {
    pub fn find_paths(&self) -> Vec<String> {
        let start_id = self.index["start"];
        let mut seen = HashSet::new();
        seen.insert(start_id);

        let tree = self.iterate_paths(0, start_id, seen);
        let paths = tree.list_paths();

        paths
            .iter()
            .filter(|l| self.nodes[l[l.len() - 1]].is_end())
            .map(|l| l.iter().map(|&x| self.nodes[x].name.clone()).join(","))
            .collect()
    }

    pub fn find_paths2(&self) -> Vec<String> {
        let mut final_paths: Vec<Vec<usize>> = vec![];

        for small_cave_id in self.list_small_caves() {
            let start_id = self.index["start"];
            let mut seen = HashMap::new();
            seen.insert(start_id, 1);

            let tree = self.iterate_paths2(0, start_id, seen, small_cave_id);
            let mut paths = tree.list_paths();
            final_paths.append(&mut paths);
        }

        final_paths
            .iter()
            .unique()
            .filter(|l| self.nodes[l[l.len() - 1]].is_end())
            .map(|l| l.iter().map(|&x| self.nodes[x].name.clone()).join(","))
            .collect()
    }

    fn list_small_caves(&self) -> Vec<usize> {
        self.nodes
            .iter()
            .filter(|&n| n.is_small_cave() && !n.is_end() && !n.is_start())
            .map(|x| self.index[&x.name])
            .collect()
    }

    fn get_links_for_node_id(&self, node_id: usize) -> &HashSet<usize> {
        &self.links[&node_id]
    }

    fn iterate_paths(&self, depth: usize, node_id: usize, seen: HashSet<usize>) -> Tree<usize> {
        let current_node = &self.nodes[node_id];
        if current_node.is_end() {
            return Tree::new(node_id);
        }

        let mut out = Tree::new(node_id);
        for &link_node_id in self.get_links_for_node_id(node_id) {
            let mut seen = seen.clone();
            let node = &self.nodes[link_node_id];
            if node.is_small_cave() {
                if !seen.contains(&link_node_id) {
                    seen.insert(link_node_id);
                } else {
                    continue;
                }
            }

            out.children
                .push(self.iterate_paths(depth + 1, link_node_id, seen));
        }

        out
    }

    fn iterate_paths2(
        &self,
        depth: usize,
        node_id: usize,
        seen: HashMap<usize, usize>,
        twice_id: usize,
    ) -> Tree<usize> {
        let current_node = &self.nodes[node_id];
        if current_node.is_end() {
            return Tree::new(node_id);
        }

        let mut out = Tree::new(node_id);
        for &link_node_id in self.get_links_for_node_id(node_id) {
            let mut seen = seen.clone();
            let node = &self.nodes[link_node_id];
            if node.is_small_cave() {
                if seen.contains_key(&link_node_id) {
                    if link_node_id == twice_id {
                        if seen[&link_node_id] > 1 {
                            continue;
                        } else {
                            seen.insert(link_node_id, seen[&link_node_id] + 1);
                        }
                    } else {
                        continue;
                    }
                } else {
                    seen.insert(link_node_id, 1);
                }
            }

            out.children
                .push(self.iterate_paths2(depth + 1, link_node_id, seen, twice_id));
        }

        out
    }
}

impl From<&[&str]> for Graph {
    fn from(s: &[&str]) -> Self {
        let mut index = HashMap::new();
        let mut nodes = vec![];
        let mut links = HashMap::new();

        for &line in s {
            let mut split = line.split('-');
            let a: String = split.next().unwrap().into();
            let b: String = split.next().unwrap().into();

            let a_pos = if !index.contains_key(&a) {
                let pos = nodes.len();
                nodes.push(Node::new(a.clone()));
                index.insert(a.clone(), pos);
                pos
            } else {
                index[&a]
            };

            let b_pos = if !index.contains_key(&b) {
                let pos = nodes.len();
                nodes.push(Node::new(b.clone()));
                index.insert(b.clone(), pos);
                pos
            } else {
                index[&b]
            };

            links
                .entry(a_pos)
                .or_insert_with(HashSet::new)
                .insert(b_pos);
            links
                .entry(b_pos)
                .or_insert_with(HashSet::new)
                .insert(a_pos);
        }

        Self {
            index,
            nodes,
            links,
        }
    }
}

impl Challenge for Day12 {
    fn new() -> Self {
        Self
    }

    fn run_ex1(&mut self) -> String {
        let graph = Graph::from(&parse_input_str!()[..]);
        let path = PathBuf::new().join("debug").join("day12.dot");
        save_graph_to_disk(&graph, &path);

        graph.find_paths().len().to_string()
    }

    fn run_ex2(&mut self) -> String {
        let graph = Graph::from(&parse_input_str!()[..]);
        let path = PathBuf::new().join("debug").join("day12.dot");
        save_graph_to_disk(&graph, &path);

        graph.find_paths2().len().to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{create_day_tests, days::day12::save_graph_to_disk};

    use super::Graph;

    create_day_tests!("12", "5457", "128506");

    const SAMPLE_DATA: &[&str] = &["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"];
    const LARGER_SAMPLE_DATA: &[&str] = &[
        "dc-end", "HN-start", "start-kj", "dc-start", "dc-HN", "LN-dc", "HN-end", "kj-sa", "kj-HN",
        "kj-dc",
    ];
    const EVEN_LARGER_SAMPLE_DATA: &[&str] = &[
        "fs-end", "he-DX", "fs-he", "start-DX", "pj-DX", "end-zg", "zg-sl", "zg-pj", "pj-he",
        "RW-he", "fs-DX", "pj-RW", "zg-RW", "start-pj", "he-WI", "zg-he", "pj-fs", "start-RW",
    ];

    #[test]
    fn test_sample() {
        let graph = Graph::from(SAMPLE_DATA);
        let path = PathBuf::new().join("debug").join("day12-sample.dot");
        save_graph_to_disk(&graph, &path);

        assert_eq!(graph.find_paths().len(), 10);
    }

    #[test]
    fn test_sample_2() {
        let graph = Graph::from(SAMPLE_DATA);
        let path = PathBuf::new().join("debug").join("day12-sample.dot");
        save_graph_to_disk(&graph, &path);

        let paths = graph.find_paths2();
        println!("{:#?}", paths);
        assert_eq!(paths.len(), 36);
    }

    #[test]
    fn test_larger_sample() {
        let graph = Graph::from(LARGER_SAMPLE_DATA);
        let path = PathBuf::new().join("debug").join("day12-larger-sample.dot");
        save_graph_to_disk(&graph, &path);

        assert_eq!(graph.find_paths().len(), 19);
    }

    #[test]
    fn test_larger_sample_2() {
        let graph = Graph::from(LARGER_SAMPLE_DATA);
        let path = PathBuf::new().join("debug").join("day12-larger-sample.dot");
        save_graph_to_disk(&graph, &path);

        assert_eq!(graph.find_paths2().len(), 103);
    }

    #[test]
    fn test_even_larger_sample() {
        let graph = Graph::from(EVEN_LARGER_SAMPLE_DATA);
        let path = PathBuf::new()
            .join("debug")
            .join("day12-even-larger-sample.dot");
        save_graph_to_disk(&graph, &path);

        assert_eq!(graph.find_paths().len(), 226);
    }

    #[test]
    fn test_even_larger_sample_2() {
        let graph = Graph::from(EVEN_LARGER_SAMPLE_DATA);
        let path = PathBuf::new()
            .join("debug")
            .join("day12-even-larger-sample.dot");
        save_graph_to_disk(&graph, &path);

        assert_eq!(graph.find_paths2().len(), 3509);
    }
}
