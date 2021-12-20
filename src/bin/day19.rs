use std::cmp;
use std::collections::{HashMap, HashSet};

use itertools::{Itertools, repeat_n};
use petgraph::Direction;
use petgraph::algo::min_spanning_tree;
use petgraph::data::FromElements;
use petgraph::dot::Dot;
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::prelude::{EdgeRef, StableUnGraph};

use aoc2021_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "day19";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

const OVERLAP_THRESHOLD: usize = 12;

const ORIENTATIONS: [(u8, Rotation); 24] = [
    (0, Rotation { permutations: [0, 1, 2], negations: [1, 1, 1] }),
    (1, Rotation { permutations: [0, 2, 1], negations: [1, 1, -1] }),
    (2, Rotation { permutations: [0, 1, 2], negations: [1, -1, -1] }),
    (3, Rotation { permutations: [0, 2, 1], negations: [1, -1, 1] }),
    (4, Rotation { permutations: [1, 0, 2], negations: [-1, 1, 1] }),
    (5, Rotation { permutations: [2, 0, 1], negations: [1, 1, 1] }),
    (6, Rotation { permutations: [1, 0, 2], negations: [1, 1, -1] }),
    (7, Rotation { permutations: [2, 0, 1], negations: [-1, 1, -1] }),
    (8, Rotation { permutations: [0, 1, 2], negations: [-1, -1, 1] }),
    (9, Rotation { permutations: [0, 2, 1], negations: [-1, -1, -1] }),
    (10, Rotation { permutations: [0, 1, 2], negations: [-1, 1, -1] }),
    (11, Rotation { permutations: [0, 2, 1], negations: [-1, 1, 1] }),
    (12, Rotation { permutations: [1, 0, 2], negations: [1, -1, 1] }),
    (13, Rotation { permutations: [2, 0, 1], negations: [1, -1, -1] }),
    (14, Rotation { permutations: [1, 0, 2], negations: [-1, -1, -1] }),
    (15, Rotation { permutations: [2, 0, 1], negations: [-1, -1, 1] }),
    (16, Rotation { permutations: [2, 1, 0], negations: [1, 1, -1] }),
    (17, Rotation { permutations: [1, 2, 0], negations: [-1, 1, -1] }),
    (18, Rotation { permutations: [2, 1, 0], negations: [-1, -1, -1] }),
    (19, Rotation { permutations: [1, 2, 0], negations: [1, -1, -1] }),
    (20, Rotation { permutations: [2, 1, 0], negations: [-1, 1, 1] }),
    (21, Rotation { permutations: [1, 2, 0], negations: [-1, -1, 1] }),
    (22, Rotation { permutations: [2, 1, 0], negations: [1, -1, 1] }),
    (23, Rotation { permutations: [1, 2, 0], negations: [1, 1, 1] }),
];

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Rotation {
    permutations: [usize; 3],
    negations: [i32; 3],
}

impl Rotation {
    fn apply(&self, p: &Position) -> Position {
        [
            self.negations[0] * p[self.permutations[0]],
            self.negations[1] * p[self.permutations[1]],
            self.negations[2] * p[self.permutations[2]],
        ]
    }

    fn permute<T: Copy>(&self, p: &[T; 3]) -> [T; 3] {
        [
            p[self.permutations[0]],
            p[self.permutations[1]],
            p[self.permutations[2]],
        ]
    }

    fn negate(&self, p: &Position) -> Position {
        [
            self.negations[0] * p[0],
            self.negations[1] * p[1],
            self.negations[2] * p[2],
        ]
    }

    fn inverse(&self) -> Self {
        let mut p = [None; 3];
        for i in 0..3 {
            p[self.permutations[i]] = Some(i);
        }

        let mut r = Rotation {
            negations: [1, 1, 1],
            permutations: p.map(Option::unwrap),
        };

        r.negations = r.permute(&self.negations);

        return r;
    }

    fn add(&self, other: &Self) -> Self {
        Rotation {
            negations: other.apply(&self.negations),
            permutations: other.permute(&self.permutations),
        }
    }
}

#[test]
pub fn test() {
    // let b0 = [-618,-824,-621];
    // let b1 = [686,422,578];
    // let s1 = [68,-1246,-43];
    // let r1 = ORIENTATIONS[10].1;
    // let b4 = [459,-707,401];
    // let s4 = add_beacon(&s1, &r1.apply(&[88, 113, -1104]));
    // let r4 = r1.add(&ORIENTATIONS[19].1);
    // let r4 = ORIENTATIONS[19].1.add(&r1);
    // assert_eq!(add_beacon(&s1, &r1.apply(&b1)), b0);
    // assert_eq!(add_beacon(&s4, &r4.apply(&[-660,-479,-426])), b4);
    // println!("{}",ORIENTATIONS.iter().map(|(_, r)| r.apply(&[1,2,3])).map(|[x,_,y]| format!("{:?}",(x,y))).join("\n"));
    panic!();
}

type Position = [i32; 3];
type Scanner = [Vec<(Position, Rotation, HashSet<Position>)>; 24];


fn relative_beacon(b: &Position, br: &Position) -> Position {
    [br[0] - b[0], br[1] - b[1], br[2] - b[2]]
}

fn subtract_beacon(b: &Position, br: &Position) -> Position {
    [b[0] - br[0], b[1] - br[1], b[2] - br[2]]
}
fn add_beacon(b: &Position, br: &Position) -> Position {
    [br[0] + b[0], br[1] + b[1], br[2] + b[2]]
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let mut scanners: Vec<Scanner> = Vec::new();
    let mut beacon_list: Vec<Position> = Vec::new();
    let mut s = [Vec::new(), Vec::new(), Vec::new()];
    let mut i = 0;
    for line in input.into_iter().chain([&String::from("")]) {
        if line == "" {
            if !beacon_list.is_empty() {
                let relative_beacons: Vec<(_, HashSet<_>)> = beacon_list.iter()
                    .map(|b| (*b, beacon_list.iter()
                        .map(|br| relative_beacon(b, br))
                        .collect::<HashSet<_>>()))
                    .collect();

                let rotated_relative_beacons = (&ORIENTATIONS)
                    .map(|(i, r)|
                        relative_beacons.iter().map(|(b, bl)| (
                            r.apply(b),
                            r,
                            bl.iter()
                                .map(|b| r.apply(b))
                                .collect::<HashSet<_>>())
                        ).collect::<Vec<_>>()
                    );

                scanners.push(rotated_relative_beacons);
                beacon_list.clear();

                println!("x_{{{}}}={:?}", i, s[0]);
                println!("y_{{{}}}={:?}", i, s[1]);
                println!("z_{{{}}}={:?}", i, s[2]);
                println!("(x_{{{}}},y_{{{}}})", i, i);
                println!("(x_{{{}}},z_{{{}}})", i, i);
                s = [Vec::new(), Vec::new(), Vec::new()];
                i += 1;
            }
        } else if line.chars().nth(1).unwrap() != '-' {
            let (x, y, z) = line.split(",")
                .map(str::parse)
                .map(Result::unwrap)
                .next_tuple().unwrap();
            beacon_list.push([x, y, z]);
            s[0].push(x);
            s[1].push(y);
            s[2].push(z);
        }
    }

    let mut pairs = vec![];


    for i in 0..scanners.len() {
        let s1 = &scanners[i];
        for j in (i + 1)..scanners.len() {
            let s2 = &scanners[j];
            if let Some(((b1, _, _), (b2, r, _))) = s1[0].iter()
                .cartesian_product(s2.iter().flat_map(|v| v.iter()))
                .find(|((_b1, _r1, bl1), (_b2, _r2, bl2))| {
                    bl1.intersection(bl2).count() >= OVERLAP_THRESHOLD
                }) {
                pairs.push(((i, j), *r, relative_beacon(b2, b1)));
            };
        }
    };

    let mut g = UnGraph::<usize, (_, Position)>::new_undirected();

    for ((s1, s2), r, b) in pairs.iter() {
        while cmp::max(*s1, *s2) >= g.node_count() {
            g.add_node(g.node_count());
        }
        g.add_edge(NodeIndex::new(*s1), NodeIndex::new(*s2), (*r, *b));
    }

    let mut g = StableUnGraph::<_, _>::from_elements(min_spanning_tree(&g));
    let mut seen_beacons = HashSet::new();
    let mut scanners_absolutes = repeat_n([0,0,0], scanners.len()).collect();
    let rotation_map = ORIENTATIONS.iter().map(|(i, r)| (r.apply(&[1, 2, 3]), *i)).collect();
    println!("{:?}", Dot::with_config(&g, &[]));
    count_unique_beacons(&mut g, NodeIndex::new(0), &mut seen_beacons, &mut scanners_absolutes, &scanners, &rotation_map, ORIENTATIONS[0].1, [0, 0, 0]);

    // eprintln!("{:?}", pairs);
    println!("{}", seen_beacons.iter().sorted().map(|x| format!("{:?}", x)).join("\n"));
    println!("{:?}", scanners_absolutes);

    let mut max_dist = 0;
    for i in 0..scanners.len() {
        let s1 = &scanners_absolutes[i];
        for j in (i + 1)..scanners.len() {
            let s2 = &scanners_absolutes[j];
            max_dist = cmp::max(max_dist,
                                (s1[0] - s2[0]).abs() +
                                    (s1[1] - s2[1]).abs() +
                                    (s1[2] - s2[2]).abs())
        }
    }

    Ok([Some(seen_beacons.len().to_string()), Some(max_dist.to_string())])
}

fn count_unique_beacons(g: &mut StableUnGraph<usize, (Rotation, Position)>, start: NodeIndex,
                        seen_beacons: &mut HashSet<Position>, scanners_absolutes: &mut Vec<Position>,
                        scanners: &Vec<Scanner>, rotation_map: &HashMap<Position, u8>,
                        rotation: Rotation, position: Position) {
    let edges: Vec<_> = g.edges_directed(start, Direction::Outgoing)
        .map(|er| (*er.weight(), er.target())).collect();
    let s = g.remove_node(start).unwrap();
    scanners_absolutes[s] = position;
    for (beacon, _, _) in scanners[s][rotation_map[&rotation.apply(&[1,2,3])] as usize].iter() {
        let absolute_beacon = add_beacon(&position, beacon);
        seen_beacons.insert(absolute_beacon);
    };
    for ((r, p), nx) in edges {
        let (rotation, position) = if start < nx {
            (r.add(&rotation), add_beacon(&position, &rotation.apply(&p)))
        } else {
            let rotation = r.inverse().add(&rotation);
            (rotation, subtract_beacon(&position, &rotation.apply(&p)))
        };
        count_unique_beacons(g, nx, seen_beacons, scanners_absolutes, scanners, rotation_map, rotation, position);
    }
}

#[test]
pub fn test_day19() {
    assert!(common::run_test(DAY, &run))
}