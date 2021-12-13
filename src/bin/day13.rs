use aoc2021_niels_overkamp::common::{self, AOCResult};

use std::collections::{HashSet};
use itertools::Itertools;

const DAY: &str = "day13";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}


pub fn run(input: &Vec<String>) -> AOCResult {
    let mut sheet = HashSet::new();
    let mut inp_iter = input.into_iter();
    for line in &mut inp_iter {
        if line == "" {
            break
        }
        let (x,y): (u32, u32) = line.split(",")
            .map(str::parse)
            .map(Result::unwrap)
            .next_tuple().unwrap();
        sheet.insert((x,y));
    }

    let fold_x = |fx, sheet| {
        let mut folded = HashSet::new();
        for (x,y) in sheet {
            if x > fx {
                folded.insert((2*fx - x, y));
            } else {
                folded.insert((x, y));
            };
        }
        folded
    };

    let fold_y = |fy, sheet| {
        let mut folded = HashSet::new();
        for (x,y) in sheet {
            if y > fy {
                folded.insert((x, 2*fy - y));
            } else {
                folded.insert((x, y));
            };
        }
        folded
    };

    let mut res1 = None;

    for fold_line in inp_iter {
        let (s, n): (&str, &str) = fold_line.split("=")
            .next_tuple()
            .unwrap();
        let n: u32 = n.parse().unwrap();
        sheet = match (s,n) {
            ("fold along x", n) => fold_x(n, sheet),
            ("fold along y", n) => fold_y(n, sheet),
            _ => panic!("Unexpected input {}={}", s, n)
        };
        if res1.is_none() {
            res1 = Some(sheet.len().to_string());
        }
    }

    let mx = (*sheet.iter().max_by(|p1, p2| (**p1).0.cmp(&(**p2).0))
        .unwrap()).0;
    let my = (*sheet.iter().max_by(|p1, p2| (**p1).1.cmp(&(**p2).1))
        .unwrap()).1;

    let mut s = String::new();
    for y in 0..my+1 {
        for x in 0..mx+1 {
            if sheet.contains(&(x,y)) {
                s.push('#');
            } else {
                s.push('.');
            }
        }
        s.push('\n');
    }

    Ok([res1, Some(s)])
}

#[test]
pub fn test_day13() {
    assert!(common::run_test(DAY, &run))
}