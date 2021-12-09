use std::cmp::{max, min};
use std::collections::HashMap;

use itertools::Itertools;

use aoc2021_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "day5";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let parsed_input: Vec<(i64, i64, i64, i64)> = input.iter().map(|l| l.split(" -> ")
        .flat_map(|c| c.split(","))
        .map(|n| n.parse().unwrap())
        .next_tuple::<(i64, i64, i64, i64)>()
        .unwrap()
    ).collect();

    let mut map = HashMap::new();

    for (x0, y0, x1, y1) in parsed_input.iter() {
        if x0 == x1 {
            for y in *min(y0, y1)..max(y0, y1) + 1 {
                map.insert((*x0, y), map.get(&(*x0, y)).unwrap_or(&0) + 1);
            }
        } else if y0 == y1 {
            for x in *min(x0, x1)..max(x0, x1) + 1 {
                map.insert((x, *y0), map.get(&(x, *y0)).unwrap_or(&0) + 1);
            }
        }
    }

    let res1 = map.values().filter(|v| **v > 1).count();

    for (x0, y0, x1, y1) in parsed_input.iter() {
        if x0 != x1 && y0 != y1 {
            let x_sign = (x1 - x0).signum();
            let y_sign = (y1 - y0).signum();
            for i in 0..((x1 - x0) * x_sign + 1) {
                let k = (x0 + x_sign * i, y0 + y_sign * i);
                map.insert(k, map.get(&k).unwrap_or(&0) + 1);
            }
        }
    }

    let res2 = map.values().filter(|v| **v > 1).count();

    Ok([Some(res1.to_string()), Some(res2.to_string())])
}

#[test]
pub fn test_day5() {
    assert!(common::run_test(DAY, &run))
}