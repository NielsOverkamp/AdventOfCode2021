use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use itertools::{Itertools};
use petgraph::{Direction, Graph};
use petgraph::dot::Dot;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::prelude::EdgeRef;
use aoc2021_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "day14";

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Pattern {
    Rec((char, char)),
    RecEnd((char, char)),
    Ref((char, char), usize),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum EdgeSide {
    Left,
    Right
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

fn hash(c: char) -> usize {
    c.to_digit(36).unwrap() as usize - 10
}

fn walk1<'a>(g: &mut Graph<Pattern, EdgeSide>, ix: NodeIndex, static_mappings: &HashMap<(char, char), char>, mappings: &'a mut HashMap<(char, char), Vec<String>>) -> (bool, String) {
    let pat = g.index_mut(ix);
    match *pat {
        Pattern::Rec(pair) => {
            let (e1, e2): (_, _) = g.edges_directed(ix, Direction::Outgoing).next_tuple().unwrap();
            let (l, r) = match *e1.weight() {
                EdgeSide::Left => (e1.target(), e2.target()),
                EdgeSide::Right => (e2.target(), e1.target())
            };
            let ((lb, mut l), (rb, r)) = (walk1(g, l, static_mappings, mappings), walk1(g, r, static_mappings, mappings));

            l.push(static_mappings[&pair]);
            l.push_str(&r);

            let v = mappings.get_mut(&pair).unwrap();
            v.push(l);

            (lb || rb, v[v.len()-1].clone())
        },
        Pattern::RecEnd(pair) => {
            if mappings.contains_key(&pair) {
                *pat = Pattern::Ref(pair, 1);
                (false, mappings[&pair][0].clone())
            } else {
                *pat = Pattern::Rec(pair);
                let mid = static_mappings[&pair];
                let l = g.add_node(Pattern::RecEnd((pair.0, mid)));
                let r = g.add_node(Pattern::RecEnd((mid, pair.1)));
                g.add_edge(ix, l, EdgeSide::Left);
                g.add_edge(ix, r, EdgeSide::Right);
                let c = String::from(static_mappings[&pair]);
                mappings.insert(pair, vec![c]);
                (true, mappings[&pair][0].clone())
            }
        },
        Pattern::Ref(pair, i) => {
            *pat = Pattern::Ref(pair, i + 1);
            (false, mappings[&pair][i].clone())
        }
    }
}

fn walk2<'a>(g: &mut Graph<Pattern, EdgeSide>, ix: NodeIndex, static_mappings: &HashMap<(char, char), char>, mappings: &'a mut HashMap<(char, char), Vec<Vec<u64>>>) -> Vec<u64> {
    let pat = g.index_mut(ix);
    match *pat {
        Pattern::Rec(pair) => {
            let (e1, e2): (_, _) = g.edges_directed(ix, Direction::Outgoing).next_tuple().unwrap();
            let (l, r) = match *e1.weight() {
                EdgeSide::Left => (e1.target(), e2.target()),
                EdgeSide::Right => (e2.target(), e1.target())
            };
            let (mut l, r) = (walk2(g, l, static_mappings, mappings), walk2(g, r, static_mappings, mappings));
            for i in 0..26 {
                l[i] += r[i];
            }
            l[hash(static_mappings[&pair])] += 1;


            let v = mappings.get_mut(&pair).unwrap();
            v.push(l);

            mappings[&pair].last().unwrap().clone()
        }
        Pattern::RecEnd(_) => panic!(),
        Pattern::Ref(pair, i) => {
            *pat = Pattern::Ref(pair, i + 1);
            mappings[&pair][i].clone()
        }
    }
}

pub fn run<'a>(input: &Vec<String>) -> AOCResult {
    let mut g = DiGraph::<Pattern, EdgeSide>::new();

    let mut iter = input[0].chars();
    let mut root_nodes = Vec::new();
    let mut prev_c = iter.next().unwrap();
    for c in iter{
        root_nodes.push(g.add_node(Pattern::RecEnd((prev_c, c))));
        prev_c = c;
    }

    let static_mappings: HashMap<(char, char), char> = input[2..].iter().map(|line| {
        let (pair, ins): (&str, &str) = line.split(" -> ")
            .next_tuple().unwrap();

        let pair: (char, char) = pair.chars().next_tuple().unwrap();

        (pair, ins.chars().next().unwrap())
    }).collect();

    let mut mappings = HashMap::new();

    println!("{:?} {:?}", root_nodes, g.index(root_nodes[0]));
    let mut last_iteration = 39;

    for iteration in 0..(40 as u32) {
        if !root_nodes.iter().fold(false, |acc, ix| {
            walk1(&mut g, *ix, &static_mappings, &mut mappings).0 || acc
        }) {
            last_iteration = iteration;
            break;
        }
    }

    println!("{:?}", last_iteration);

    println!("{:?}", Dot::new(&g));

    let mut count_mappings: HashMap<(char, char), Vec<Vec<u64>>> = mappings.into_iter().map(|(pair, s_vec)| {
        let counts_vec = s_vec.into_iter().map(|s| {
            let mut counts: Vec<u64> = vec![0; 26];
            for c in s.chars() {
                counts[hash(c)] += 1
            }
            counts
        }).collect();
        (pair, counts_vec)
    }).collect();

    for _ in last_iteration..40 {
        for ix in root_nodes.iter() {
            walk2(&mut g, *ix, &static_mappings, &mut count_mappings);
        }
    }

    let mut total_count10 = vec![0;26];
    let mut total_count40 = vec![0;26];

    if let Pattern::Rec((first_char, _)) = *g.index(root_nodes[0]) {
        total_count10[hash(first_char)] += 1;
        total_count40[hash(first_char)] += 1;
    } else {
        panic!()
    };

    for ix in root_nodes.iter() {
        let (vec10, vec40, pair) = match *g.index(*ix) {
            Pattern::Rec(pair) => {
                println!("rec");
                let vec = &count_mappings[&pair];
                (&vec[9], &vec[39], pair)
            }
            Pattern::RecEnd(_) => panic!(),
            Pattern::Ref(pair, i) => {
                println!("ref {}", i);
                let vec = &count_mappings[&pair];
                (&vec[i-32], &vec[i-2], pair)
            }
        };

        total_count10[hash(pair.1)] += 1;
        total_count40[hash(pair.1)] += 1;
        
        for i in 0..26 {
            total_count10[i] += vec10[i];
            total_count40[i] += vec40[i];
        }
        println!("{:?} {:?}", vec10, vec40);
        println!("{:?} {:?}", total_count10, total_count40);
    }


    let res1 = total_count10.into_iter().filter(|n| *n > 0).minmax().into_option().unwrap();
    let res1 = res1.1 - res1.0;
    let res2 = total_count40.into_iter().filter(|n| *n > 0).minmax().into_option().unwrap();
    let res2 = res2.1 - res2.0;

    Ok([Some(res1.to_string()), Some(res2.to_string())])
}

#[test]
pub fn test_day14() {
    assert!(common::run_test(DAY, &run))
}