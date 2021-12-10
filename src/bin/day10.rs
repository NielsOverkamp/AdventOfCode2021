use aoc2021_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "day10";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let mut res1 = 0;
    let mut autocomplete_scores = Vec::new();
    for line in input {
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '<' | '{' | '[' => stack.push(c),
                ')' | '>' | '}' | ']' => {
                    match (c, stack.pop()) {
                        (_, None) => {
                            break
                        },
                        (')', Some('(')) | ('>', Some('<')) | ('}', Some('{')) | (']', Some('[')) => (),
                        (illegal_c, _) => {
                            stack.clear();
                            res1 += match illegal_c {
                                ')' => 3,
                                '>' => 25137,
                                '}' => 1197,
                                ']' => 57,
                                _ => 0
                            };
                            break
                        }
                    }
                }
                _ => ()
            }
        }
        let mut score: i64 = 0;
        loop {
            match stack.pop() {
                None => break,
                Some('(') => score = score * 5 + 1,
                Some('<') => score = score * 5 + 4,
                Some('{') => score = score * 5 + 3,
                Some('[') => score = score * 5 + 2,
                _ => ()
            }
        }
        if score > 0 {
            autocomplete_scores.push(score);
        }
    }
    autocomplete_scores.sort();
    let res2 = autocomplete_scores[autocomplete_scores.len() / 2];
    Ok([Some(res1.to_string()), Some(res2.to_string())])
}

#[test]
pub fn test_day10() {
    assert!(common::run_test(DAY, &run))
}