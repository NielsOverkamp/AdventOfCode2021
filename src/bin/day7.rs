use itertools::Itertools;

use aoc2021_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "day7";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

fn incremental_distance_sum(position: i32, positions: &Vec<i32>) -> i32 {
    positions.iter().map(|p| {
        let diff = (position - *p).abs();
        (diff * (diff + 1)) / 2
    }).sum()
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let positions: Vec<i32> = input[0].split(",").map(|n| n.parse().unwrap()).sorted().collect();
    let optimal_position1 = positions[positions.len() / 2];
    let res1: i32 = positions.iter().map(|p| (optimal_position1 - *p).abs()).sum();


    let mut guess = ((positions.iter().sum::<i32>() as f32) / (positions.len() as f32)).round() as i32;
    let mut cost = incremental_distance_sum(guess, &positions);
    let mut new_cost = incremental_distance_sum(guess - 1, &positions);

    let direction = (new_cost - cost).signum();

    loop {
        new_cost = incremental_distance_sum(guess + direction, &positions);
        if new_cost > cost {
            break;
        }
        cost = new_cost;
        guess += direction;
    };
    Ok([Some(res1.to_string()), Some(cost.to_string())])
}

#[test]
pub fn test_day7() {
    assert!(common::run_test(DAY, &run))
}