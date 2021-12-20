use std::fmt::{Display, Formatter, Write};
use std::ops::Deref;
use itertools::Itertools;
use aoc2021_niels_overkamp::common::{self, AOCResult};

use crate::Snailfish::Leaf;

const DAY: &str = "day18";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum Snailfish {
    Leaf(u32),
    Node(Box<Node>),
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Node {
    left: Snailfish,
    right: Snailfish,
}

impl Snailfish {
    fn add(self, other: Self) -> Self {
        Snailfish::Node(Box::new(Node {
            left: self,
            right: other,
        }))
    }

    fn split(&mut self) -> bool {
        match self {
            Snailfish::Leaf(v) => {
                if *v >= 10 {
                    *self = Snailfish::Node(Box::new(Node {
                        left: Snailfish::Leaf(*v / 2),
                        right: Snailfish::Leaf((*v + 1) / 2),
                    }));
                    true
                } else {
                    false
                }
            }
            Snailfish::Node(n) => {
                n.left.split() || n.right.split()
            }
        }
    }

    fn explode(&mut self) {
        self._explode(0);
    }

    fn reduce(&mut self) {
        while self._explode(0).is_some() || self.split() {}
    }

    fn magnitude(&self) -> u32 {
        match self {
            Snailfish::Leaf(v) => *v,
            Snailfish::Node(n) => n.left.magnitude() * 3 + n.right.magnitude() * 2
        }
    }

    fn _explode(&mut self, depth: u32) -> Option<(Option<u32>, Option<u32>)> {
        if depth >= 4 {
            if let Snailfish::Node(n) = self {
                if let Node {left: Snailfish::Leaf(l), right: Snailfish::Leaf(r)} = Box::deref(n) {
                    let res = (Some(*l), Some(*r));
                    *self = Snailfish::Leaf(0);
                    return Some(res);
                }
            }
        }
        match self {
            Snailfish::Leaf(_) => None,
            Snailfish::Node(n) => {
                if let Some((l, r)) = n.left._explode(depth + 1) {
                    r.map(|r| n.right.fizzle_right(r));
                    Some((l, None))
                } else if let Some((l, r)) = n.right._explode(depth + 1) {
                    l.map(|l| n.left.fizzle_left(l));
                    Some((None, r))
                } else {
                    None
                }
            }
        }
    }

    fn fizzle_left(&mut self, ember: u32) {
        match self {
            Snailfish::Leaf(v) => { *v += ember }
            Snailfish::Node(n) => {
                n.right.fizzle_left(ember);
            }
        }
    }

    fn fizzle_right(&mut self, ember: u32) {
        match self {
            Snailfish::Leaf(v) => { *v += ember }
            Snailfish::Node(n) => {
                n.left.fizzle_right(ember);
            }
        }
    }
}

impl std::str::FromStr for Snailfish {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = vec![];
        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            match c {
                ']' => {
                    let n = Snailfish::Node(Box::new(Node {right: stack.pop().unwrap(), left: stack.pop().unwrap()}));
                    stack.push(n);
                },
                ',' | '[' => (),
                d => stack.push(Snailfish::Leaf(d.to_digit(10).unwrap())),
            };
        };
        stack.pop().ok_or(())
    }
}

impl Display for Snailfish {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Snailfish::Leaf(v) => write!(f, "{}", v),
            Snailfish::Node(n) => write!(f, "[{},{}]", n.left, n.right)
        }
    }
}

#[test]
fn test() {
    let s1 = Leaf(1).add(Leaf(1))
        .add(Leaf(2).add(Leaf(2)))
        .add(Leaf(3).add(Leaf(3)))
        .add(Leaf(4).add(Leaf(4)));

    let mut s2 = s1.clone();
    s2.explode();
    s2.split();

    assert_eq!(s1, s2);

    let mut s2 = s2.add(Leaf(5).add(Leaf(5)));
    s2.reduce();

    assert_eq!(s2, Leaf(3).add(Leaf(0))
        .add(Leaf(5).add(Leaf(3)))
        .add(Leaf(4).add(Leaf(4)))
        .add(Leaf(5).add(Leaf(5))));
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let numbers: Vec<Snailfish> = input.into_iter().map(|s| s.parse::<Snailfish>()).map(Result::unwrap).collect();

    let res1 = numbers.clone().into_iter().reduce(|n1, n2| {
        let mut n = n1.add(n2);
        n.reduce();
        n
    }).unwrap();

    let res2 = numbers.iter().cartesian_product(numbers.iter()).map(|(n1, n2)| {
        let mut n = n1.clone().add(n2.clone());
        n.reduce();
        n.magnitude()
    }).max().unwrap();
    
    Ok([Some(res1.magnitude().to_string()), Some(res2.to_string())])
}

#[test]
pub fn test_day18() {
    assert!(common::run_test(DAY, &run))
}