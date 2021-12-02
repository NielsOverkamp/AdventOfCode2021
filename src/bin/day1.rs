use aoc2021_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "day1";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let mut parsed_input = input.iter().map(|s| s.parse().expect("Parse error"));

    let res1 = parsed_input.clone()
        .fold((0,i32::MAX), |(acc, last), depth| (if last < depth {acc+1} else {acc}, depth)).0;

    let depth1 = parsed_input.next().unwrap();
    let depth2 = parsed_input.next().unwrap();
    let depth3 = parsed_input.next().unwrap();

    let res2 = parsed_input
        .fold((0,depth1+depth2+depth3, depth2+depth3, depth3), |(acc, w1, w2, w3), depth| {
            let w2 = w2 + depth;
            (if w2 > w1 {acc + 1} else {acc}, w2, w3 + depth, depth)
        }).0;
    Ok([Some(res1.to_string()), Some(res2.to_string())])
}

#[test]
pub fn test_day1() {
    assert!(common::run_test(DAY, &run))
}