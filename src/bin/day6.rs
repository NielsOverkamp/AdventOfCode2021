use aoc2021_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "day6";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let parsed_input = input[0].split(",").map(|n| n.parse::<usize>().unwrap());

    let mut fish_buckets: [i64; 9] = [0;9];

    for fish in parsed_input {
        fish_buckets[fish] += 1;
    }

    for _ in 0..80 {
        let zero_fish = fish_buckets[0];

        for i in 0..8 {
            fish_buckets[i] = fish_buckets[i+1];
        }

        fish_buckets[6] += zero_fish;
        fish_buckets[8] = zero_fish;

    }

    let res1: i64 = fish_buckets.iter().sum();

    for _ in 80..256 {
        let zero_fish = fish_buckets[0];

        for i in 0..8 {
            fish_buckets[i] = fish_buckets[i+1];
        }

        fish_buckets[6] += zero_fish;
        fish_buckets[8] = zero_fish;
    }
    let res2: i64 = fish_buckets.iter().sum();

    Ok([Some(res1.to_string()), Some(res2.to_string())])
}

#[test]
pub fn test_day6() {
    assert!(common::run_test(DAY, &run))
}