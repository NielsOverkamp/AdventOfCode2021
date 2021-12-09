use std::collections::HashSet;

use itertools::{Itertools, zip};

use aoc2021_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "day9";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let map: Vec<Vec<u8>> = input.into_iter().map(|line| line.split("").into_iter().map(|n| n.parse())
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect()).collect();
    let directions: [(i8, i8); 4] = [
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1),
    ];
    let mut res1: u32 = 0;
    let mut low_points = Vec::new();
    for (y, line) in zip(0..map.len(), map.iter()) {
        for (x, cell) in zip(0..line.len(), line) {
            if directions.iter()
                .all(|(dx, dy)| {
                    let (y, x): (i8, i8) = ((y as i8) + dy, (x as i8) + dx);
                    if x >= 0 && y >= 0 {
                        map.get(y as usize).map_or(true, |line| line.get(x as usize).map_or(true, |h| h > cell))
                    } else {
                        true
                    }
                }) {
                low_points.push((x, y));
                res1 += 1 + *cell as u32;
            }
        }
    }

    let (b1, b2, b3): (usize, usize, usize) = low_points.into_iter()
        .map(|(x, y)| {
            let mut frontier = Vec::from([(x, y)]);
            let mut visited = HashSet::new();
            let mut new_frontier = Vec::new();

            while frontier.len() > 0 {
                for (x, y) in frontier {
                    for (dx, dy) in directions {
                        let (y, x): (i8, i8) = ((y as i8) + dy, (x as i8) + dx);
                        if x >= 0 && y >= 0 {
                            let (y, x) = (y as usize, x as usize);
                            if y < map.len() &&
                                x < map[y].len() &&
                                map[y][x] != 9 &&
                                !visited.contains(&(x,y)) {
                                new_frontier.push((x,y));
                                visited.insert((x, y));
                            }
                        }
                    }
                }

                frontier = new_frontier;
                new_frontier = Vec::new();
            }
            visited.len()
        })
        .sorted_by(|p, q| q.cmp(p))
        .next_tuple().unwrap();
    Ok([Some(res1.to_string()), Some((b1 * b2 * b3).to_string())])
}

#[test]
pub fn test_day9() {
    assert!(common::run_test(DAY, &run))
}