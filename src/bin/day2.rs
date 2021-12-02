use aoc2021_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "day2";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let parsed_input = input.iter().map(|input| {
        let mut sep_iter = input.split_ascii_whitespace();
        let res: (&str, i64) = (sep_iter.next().unwrap(), sep_iter.next().unwrap().parse().unwrap());
        res
    });

    let (dist1, depth1) = parsed_input.clone().fold((0,0), |(dist,depth), (t, l)| match t {
        "forward" => (dist + l, depth),
        "down"  => (dist, depth + l),
        "up" => (dist, depth -l),
        _ => (dist, depth)
    });

    let (_, dist2, depth2) = parsed_input.clone().fold((0,0, 0), |(aim, dist,depth), (t, l)| match t {
        "forward" => (aim, dist + l, depth + l*aim),
        "down"  => (aim + l, dist, depth),
        "up" => (aim - l, dist, depth),
        _ => (aim, dist, depth)
    });
    Ok([Some((dist1*depth1).to_string()), Some((dist2*depth2).to_string())])
}

#[test]
pub fn test_day2() {
    assert!(common::run_test(DAY, &run))
}