use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use petgraph::algo::all_simple_paths;
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::prelude::EdgeRef;

use aoc2021_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "day12";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let mut nx: u32 = 1;
    let mut nx_map = HashMap::from([("start", 0), ("end",1)]);
    let mut large_caves = HashSet::new();
    let mut small_caves = HashSet::new();

    // Convert input into id pairs (edges)
    let mut edges: Vec<(_, _)> = input.into_iter().map(|l| l.split("-").map(|s| {
        match s {
            "start" => 0,
            "end" => 1,
            _ => {
                *nx_map.entry(s).or_insert_with(|| {
                    nx += 1;
                    if s.chars().all(char::is_uppercase) {
                        large_caves.insert(nx);
                    } else {
                        small_caves.insert(nx);
                    }
                    nx
                })
            }
        }
    }).next_tuple().unwrap()).collect();


    // Replace large caves edges for every pair of neighbours
    let mut large_cave_map = HashMap::new();
    let mut i = 0;
    let mut large_cave_neighbours = HashMap::new();
    while i < edges.len() {
        let (c1, c2) = edges.get(i).unwrap();
        if large_caves.contains(c1)  {
            let (c1, c2) = edges.remove(i);
            large_cave_map.entry(c1).or_insert(Vec::new()).push(c2);
            *large_cave_neighbours.entry(c2).or_insert(0) += 1;
        } else if large_caves.contains(c2) {
            let (c1, c2) = edges.remove(i);
            large_cave_map.entry(c2).or_insert(Vec::new()).push(c1);
            *large_cave_neighbours.entry(c1).or_insert(0) += 1;
        } else {
            i += 1
        }
    }
    for small_caves in large_cave_map.values() {
        for (c1, c2) in small_caves.iter().cartesian_product(small_caves.iter()) {
            if c1 < c2 {
                edges.push((*c1, *c2));
            }
        }
    }

    let mut g = UnGraph::<u32, ()>::from_edges(edges);

    let res1=  all_simple_paths::<Vec<_>, _>(&g, 0.into(), 1.into(), 0, None)
        .count();

    let mut res2 = res1;

    for c in small_caves.iter() {
        let c_ix = (*c).into();
        let double_node = g.add_node(0);

        let mut double_edges: Vec<_> = g.edges(c_ix).map(|e_ref| {
            let n = if e_ref.source() == c_ix {
                e_ref.target()
            } else {
                e_ref.source()
            };
            (double_node, n)
        }).collect();

        for _ in 0..*large_cave_neighbours.get(c).unwrap_or(&0) {
            double_edges.push((double_node, c_ix))
        }


        g.extend_with_edges(double_edges);

        res2 += all_simple_paths::<Vec<_>, _>(&g, 0.into(), 1.into(), 0, None)
            .filter(|p| {
                let ns: Vec<_> = p.into_iter().filter(|ix| **ix == c_ix || **ix == double_node).collect();
                ns.len() == 2 && ns[0] < ns[1]
            })
            .count();

        g.remove_node(double_node);
    }

    Ok([Some(res1.to_string()), Some(res2.to_string())])
}

#[test]
pub fn test_day12() {
    assert!(common::run_test(DAY, &run))
}