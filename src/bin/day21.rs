use std::fs::File;
use std::io::Write;

use petgraph::{Incoming};
use petgraph::dot::Dot;
use petgraph::prelude::{DiGraph, EdgeRef};

use aoc2021_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "day21";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let p1: u8 = input[0].split(": ").nth(1).unwrap().parse().unwrap();
    let p2: u8 = input[1].split(": ").nth(1).unwrap().parse().unwrap();
    Ok([Some(run1(p1, p2)), Some(run2(p1, p2))])
}

fn roll(i: u8, start_roll: u8) -> u8 {
    (3 * (start_roll + 6 * i + 2)) % 10
}

fn pos(i: u8, start_pos: u8, start_roll: u8) -> u8 {
    ((start_pos
        + 5 * start_roll * (i / 5)
        + (0..(i % 5)).map(|j| roll(j, start_roll)).sum::<u8>()
        - 1
    ) % 10) + 1
}

fn score(i: u32, start_pos: u8, start_roll: u8) -> u32 {
    (1..(i % 10) as u8 + 1).map(|j| pos(j, start_pos, start_roll) as u32).sum::<u32>()
        + (i / 10) * (1..11).map(|j| pos(j, start_pos, start_roll) as u32).sum::<u32>()
}

const GOAL1: u32 = 1000;

pub fn run1(p1: u8, p2: u8) -> String {
    let p1_gradient = score(10, p1, 0);
    let p2_gradient = score(10, p2, 3);


    let (p_fast, p_slow, rolls_offset) = if p1_gradient >= p2_gradient {
        ((p1, 0, p1_gradient), (p2, 3, p2_gradient), 0)
    } else {
        ((p2, 3, p2_gradient), (p1, 0, p1_gradient), 1)
    };

    let min_rolls = 10 * GOAL1 / p_fast.2;
    let mut rolls: _ = None;
    for i in 0..10 {
        if score(min_rolls + i, p_fast.0, p_fast.1) >= GOAL1 {
            rolls = Some(min_rolls + i);
            break;
        }
    };

    let res1 = (((rolls.unwrap() * 2) - 1 + rolls_offset) * 3)
        * score(rolls.unwrap() - 1 + rolls_offset, p_slow.0, p_slow.1);

    res1.to_string()
}

const GOAL2: u8 = 21;

fn length_distribution(start_pos: u8) -> [[u64; GOAL2 as usize];2] {
    let roll_distribution = [[3, 1], [4, 3], [5, 6], [6, 7], [7, 6], [8, 3], [9, 1]];
    let start = (start_pos, 0u8);
    let mut g = DiGraph::<(u8, u8), u8>::new();
    let start_index = g.add_node(start);
    let mut frontier = vec![(start_pos, 0u8, start_index)];
    let mut visited = [None; 10 * 31];
    let hash = |p: u8, s: u8| (s as usize) * 10 + (p - 1) as usize;
    visited[hash(start_pos, 0)] = Some(start_index);

    while let Some((p, s, i)) = frontier.pop() {
        for [r, d] in roll_distribution {
            let np = (p + r - 1) % 10 + 1;
            let ns = s + np;
            let ni = visited[hash(np, ns)].unwrap_or_else(|| {
                let ni = g.add_node((np, ns));
                visited[hash(np, ns)] = Some(ni);
                if ns < GOAL2 {
                    frontier.push((np, ns, ni));
                }
                ni
            });
            g.add_edge(i, ni, d);
        }
    }

    if let Ok(mut f) = File::create(format!("debug_outputs/day21-{}.dot", start_pos)) {
        write!(f, "{:?}", Dot::new(&g)).unwrap();
    }

    let indices = visited;
    let mut visited = [None; 10 * (GOAL2 as usize + 10)];
    let mut start = [0u64; GOAL2 as usize];
    start[0] = 1;
    visited[0] = Some(start);
    for i in &indices[10..] {
        if let Some(i) = *i {
            // println!("{} {:?}", i.index(), g.node_weight(i));
            let mut l_dis = [0; GOAL2 as usize];
            for e in g.edges_directed(i, Incoming) {
                let p_l_dis = &visited[e.source().index()].unwrap();
                // println!("\t{:?}: {:?}", g.node_weight(e.source()), p_l_dis);
                for s in 0..(GOAL2 as usize) - 1 {
                    l_dis[s + 1] += p_l_dis[s] * (*e.weight() as u64);
                }
            }
            // println!("\t\t{:?}", l_dis);
            visited[i.index()] = Some(l_dis);
        }
    }
    let mut winning_dis = [0; GOAL2 as usize];
    let mut losing_dis = [0; GOAL2 as usize];
    for i in indices {
        if let Some(i) = i {
            let l_dis = if (*g.node_weight(i).unwrap()).1 >= GOAL2 {
                // print!("win ");
                &mut winning_dis
            } else {
                // print!("lose ");
                &mut losing_dis
            };
            let p_l_dis = visited[i.index()].unwrap();
            // println!("{:?}, {:?}", g.node_weight(i), p_l_dis);
            for s in 0..(GOAL2 as usize) {
                l_dis[s] += p_l_dis[s];
            }
        }
    }
    [losing_dis,winning_dis]
}

pub fn run2(p1: u8, p2: u8) -> String {
    let [_, w4] = length_distribution(p1);
    let [l8, _] = length_distribution(p2);
    w4[1..].iter().zip(l8).map(|(w4, l8)| w4*l8).sum::<u64>().to_string()
}

#[test]
pub fn test_day21() {
    assert!(common::run_test(DAY, &run))
}
