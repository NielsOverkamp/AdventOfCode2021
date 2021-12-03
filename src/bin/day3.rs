use aoc2021_niels_overkamp::common::{self, AOCResult};
use itertools::partition;

const DAY: &str = "day3";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let max_l: i64 = input.iter().reduce(|a, b| if a.len() > b.len() { a } else { b }).unwrap().len() as i64;
    let parsed_input: Vec<i64> = input.iter().map(|input| i64::from_str_radix(input, 2).unwrap()).collect();

    let mut o2_partition = parsed_input.clone();
    let mut co2_partition = parsed_input;

    for i in 0..max_l {
        let i = max_l - i - 1;
        if o2_partition.len() > 1 {
            let o2_split_index = partition(&mut o2_partition, |n| (*n >> i) & 1 == 1);
            if o2_split_index * 2 >= o2_partition.len()  {
                o2_partition.drain(o2_split_index..);
            } else {
                o2_partition.drain(0..o2_split_index);
            }
        }

        if co2_partition.len() > 1 {
            let co2_split_index = partition(&mut co2_partition, |n| (*n >> i) & 1 == 0);
            if co2_split_index * 2 > co2_partition.len() {
                co2_partition.drain(0..co2_split_index);
            } else {
                co2_partition.drain(co2_split_index..);
            }
        }
    }

    println!("{:?} {:?}", o2_partition, co2_partition);

    Ok([None, Some((o2_partition[0] * co2_partition[0]).to_string())])
}

#[test]
pub fn test_day3() {
    assert!(common::run_test(DAY, &run))
}