use itertools::{Itertools, zip};

use aoc2021_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "day8";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

fn count_signals(code: &u8) -> u8 {
    (0..7).map(|i| *code >> i & 1).sum()
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let a_byte = b'a';
    let parsed_input: Vec<(Vec<u8>, Vec<u8>)> = input.into_iter().map(|line|
        line.split(" | ").map(|codes|
            codes.split(" ").map(|code| {
                code.bytes().fold(0, |b, char| b + (1 << (char - a_byte)))
            }).collect::<Vec<u8>>()
        ).next_tuple().unwrap()
    ).collect();

    let res1 = parsed_input.iter()
        .flat_map(|(_, output)| output.iter()
            .map(&count_signals))
        .filter(|n| *n == 2 || *n == 4 || *n == 3 || *n == 7)
        .count();

    let mut res2: u32 = 0;
    for (input, output) in parsed_input.iter() {
        let mut acc = [Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new()];
        for signal in input {
            acc[count_signals(signal) as usize].push(*signal)
        }
        let (one, four, seven, eight) = (acc[2][0], acc[4][0], acc[3][0], acc[7][0]);
        let two = *acc[5].iter().filter(|signal| count_signals(&(*signal & four)) == 2).next().unwrap();
        let c = two & one;
        let five = *acc[5].iter().filter(|signal| *signal & c == 0).next().unwrap();
        let three = *acc[5].iter().filter(|signal| *signal & one == one).next().unwrap();
        let d = three & four - one;
        let zero = *acc[6].iter().filter(|signal| *signal & d == 0).next().unwrap();
        let six = *acc[6].iter().filter(|signal| *signal & c == 0).next().unwrap();
        let nine = *acc[6].iter().filter(|signal| *signal & (c+d) == c + d).next().unwrap();

        let codes = [(0,zero), (1, one), (2,two), (3,three), (4, four), (5, five), (6,six), (7,seven), (8,eight), (9,nine)];
        for (dig_i, signal) in zip(0..4, output) {
            res2 += codes.iter().filter(|(i, c)| c == signal).next().unwrap().0 * 10_u32.pow(3-dig_i);
        }
    }

    Ok([Some(res1.to_string()), Some(res2.to_string())])
}

#[test]
pub fn test_day8() {
    assert!(common::run_test(DAY, &run))
}