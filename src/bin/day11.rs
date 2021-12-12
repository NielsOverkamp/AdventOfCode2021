use itertools::Itertools;

use aoc2021_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "day11";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

const DIRS: [(i32, i32); 8] = [
    (0,1),
    (1,1),
    (1,0),
    (1,-1),
    (0,-1),
    (-1,-1),
    (-1,0),
    (-1,1),
];

pub fn run(input: &Vec<String>) -> AOCResult {
    let parsed_input: Vec<Vec<u32>> = input.into_iter().map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();
    let mut grid = [[0; 10]; 10];
    for (i, j) in (0..10).into_iter().cartesian_product(0..10) {
        grid[i][j] = parsed_input[i][j]
    }

    let neighbours = |i, j| DIRS.iter()
        .map(move |(di, dj)| (i as i32 + *di, j as i32 + *dj))
        .filter(|(i, j)| *i >= 0 && *j >= 0 && *i <10 && *j < 10)
        .map(move |(i, j)| (i as usize, j as usize));

    let mut res1 = 0;
    let mut res2 = None;
    for n in 0..u32::MAX {
        let mut has_flashed = true;
        let mut new_grid = grid.clone();
        for (i, j) in (0..10).into_iter().cartesian_product(0..10) {
            new_grid[i][j] += 1;
        }
        let mut flash_count = 0;
        while has_flashed {
            has_flashed = false;

            for (i, j) in (0..10).into_iter().cartesian_product(0..10) {
                if new_grid[i][j] > 9 {
                    for (ni, nj) in neighbours(i,j) {
                        if new_grid[ni][nj] != 0 {
                            new_grid[ni][nj] += 1;
                        }
                    }
                    new_grid[i][j] = 0;
                    flash_count += 1;
                    has_flashed = true;
                }
            }
            grid = new_grid;
        }
        if n < 100 {
            res1 += flash_count;
        } else if flash_count == 100 {
            res2 = Some(n + 1);
            break;
        }
    }
    Ok([Some(res1.to_string()), res2.map(|n| n.to_string())])
}

#[test]
pub fn test_day11() {
    assert!(common::run_test(DAY, &run))
}