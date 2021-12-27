use aoc2021_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "day21";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

fn roll(i: u8, start_roll: u8) -> u8 {
    (3 * (start_roll + 6 * i + 2)) % 10
}

fn pos(i: u8, start_pos: u8, start_roll: u8) -> u8 {
    ((start_pos
        + 5 * start_roll * (i / 5)
        + (0..(i % 5)).map(|j| roll(j, start_roll)).sum::<u8>()
        - 1
    ) % 10) + 1
}

fn score(i: u32, start_pos: u8, start_roll: u8) -> u32 {
    (1..(i % 10) as u8 + 1).map(|j| pos(j, start_pos, start_roll) as u32).sum::<u32>()
        + (i / 10) * (1..11).map(|j| pos(j, start_pos, start_roll) as u32).sum::<u32>()
}

const GOAL: u32 = 1000;

pub fn run(input: &Vec<String>) -> AOCResult {
    let p1: u8 = input[0].split(": ").nth(1).unwrap().parse().unwrap();
    let p2: u8 = input[1].split(": ").nth(1).unwrap().parse().unwrap();


    let p1_gradient = score(10, p1, 0);
    let p2_gradient = score(10, p2, 3);



    let (p_fast, p_slow, rolls_offset) = if p1_gradient >= p2_gradient {
        ((p1, 0, p1_gradient), (p2, 3, p2_gradient), 0)
    } else {
        ((p2, 3, p2_gradient), (p1, 0, p1_gradient), 1)
    };

    let min_rolls = 10 * GOAL / p_fast.2;
    let mut rolls: _ = None;
    for i in 0..10 {
        if score(min_rolls + i, p_fast.0, p_fast.1) >= GOAL {
            rolls = Some(min_rolls + i);
            break;
        }
    };

    let res1 = (((rolls.unwrap() * 2) -1 + rolls_offset) * 3)
        * score(rolls.unwrap() - 1 + rolls_offset, p_slow.0, p_slow.1);

    Ok([Some(res1.to_string()), None])
}

#[test]
pub fn test_day21() {
    assert!(common::run_test(DAY, &run))
}