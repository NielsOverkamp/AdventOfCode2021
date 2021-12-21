use std::cmp;
use std::collections::HashMap;
use std::process::id;
use itertools::Itertools;

use aoc2021_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "day20";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let mut algorithm = [false; 512];
    let bytes = input[0].as_bytes();
    for i in 0usize..512usize {
        algorithm[i] = bytes[i] == b'#';
    }

    let algorithm = algorithm;

    let mut image = HashMap::new();

    let mut minmax = [0; 4];

    let minmax_cmp = |i, j, [min_i, max_i, min_j, max_j]: [i32;4]| [
        cmp::min(i, min_i),
        cmp::max(i, max_i),
        cmp::min(j, min_j),
        cmp::max(j, max_j),
    ];

    for (i, line) in (0i32..).zip(input[2..].iter()) {
        for (j, c) in (0i32..).zip(line.chars()) {
            minmax = minmax_cmp(i, j, minmax);
            image.insert((i, j), c == '#');
        }
    }

    let neighbours = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 0), (0, 1), (1, -1), (1, 0), (1, 1)];

    let mut default_pixel = false;

    let mut res1 = 0;
    let mut res2 = 0;

    for i in 0..50 {
        let mut new_image = HashMap::new();
        let mut new_minmax = [0; 4];
        for i in minmax[0] - 1..minmax[1] + 2 {
            for j in minmax[2] - 1..minmax[3] + 2 {
                new_minmax = minmax_cmp(i, j, new_minmax);
                new_image.insert((i, j), algorithm[
                    neighbours.iter()
                        .fold(0, |acc, (di, dj)| {
                            if *image.get(&(di + i, dj + j)).unwrap_or(&default_pixel) {
                                (acc << 1) + 1
                            } else {
                                acc << 1
                            }
                        })
                    ],
                );
            }
        }

        if default_pixel {
            default_pixel = algorithm[511]
        } else {
            default_pixel = algorithm[0]
        }

        image = new_image;
        minmax = new_minmax;

        if i == 1 {
            res1 = image.values().filter(|x| **x).count();
        } else if i == 49 {
            res2 = image.values().filter(|x| **x).count();
        }

    }


    Ok([Some(res1.to_string()), Some(res2.to_string())])
}

#[test]
pub fn test_day20() {
    assert!(common::run_test(DAY, &run))
}