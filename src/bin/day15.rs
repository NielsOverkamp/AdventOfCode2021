use std::ops::Index;

use petgraph::algo::astar::astar;
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::prelude::EdgeRef;

use aoc2021_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "day15";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let mut g = UnGraph::<usize, ()>::new_undirected();

    let l = input[0].len();

    for line in input.iter() {
        if line == "" {
            break;
        }
        for c in line.chars() {
            let ix = g.add_node(c.to_digit(10).unwrap() as usize);
            let n = ix.index();
            if n >= 1 && n % l != 0 {
                g.add_edge(ix, NodeIndex::new(n - 1), ());
            }
            if n >= l {
                g.add_edge(ix, NodeIndex::new(n - l), ());
            }
        }
    }

    let goal_node = NodeIndex::new(l * l - 1);


    let res1 = if let Some((cost, _)) = astar::<_, _, _, usize, _>(
        &g,
        NodeIndex::new(0),
        |ix| ix == goal_node,
        |e| *g.index(e.target()),
        |ix| (l - 1 - (ix.index() % l) + l - 1 - (ix.index() / l)))
    {
        cost.to_string()
    } else {
        panic!()
    };

    let mut g = UnGraph::<usize, ()>::new_undirected();
    let rep = 5;
    let l = l * 5;

    for i in 0..rep {
        for line in input.iter() {
            if line == "" {
                break;
            }
            for j in 0..rep {
                for c in line.chars() {
                    let weight = (c.to_digit(10).unwrap() as usize + i + j - 1) % 9 + 1;
                    let ix = g.add_node(weight);
                    let n = ix.index();
                    if n >= 1 && n % l != 0 {
                        g.add_edge(ix, NodeIndex::new(n - 1), ());
                    }
                    if n >= l {
                        g.add_edge(ix, NodeIndex::new(n - l), ());
                    }
                }
            }
        }
    }

    let goal_node = NodeIndex::new(l * l - 1);

    let res2 = if let Some((cost, _)) = astar::<_, _, _, usize, _>(
        &g,
        NodeIndex::new(0),
        |ix| ix == goal_node,
        |e| *g.index(e.target()),
        |ix| (l - 1 - (ix.index() % l) + l - 1 - (ix.index() / l)))
    {
        cost.to_string()
    } else {
        panic!()
    };

    Ok([Some(res1), Some(res2)])
}

#[test]
pub fn test_day15() {
    assert!(common::run_test(DAY, &run))
}