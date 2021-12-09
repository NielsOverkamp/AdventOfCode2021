use aoc2021_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "day4";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}


pub fn run(input: &Vec<String>) -> AOCResult {
    let mut calls: Vec<i32> = Vec::new();
    for n in input[0].split(",") {
        calls.push(n.parse().unwrap())
    }

    let card_width = 5;
    let cards_n = (input.len() - 2) / (card_width + 1);

    let mut cards: Vec<Vec<Vec<Option<i32>>>> = Vec::with_capacity(cards_n);

    for i in 0..cards_n {
        let cursor = 2 + i * (card_width + 1);
        cards.push(input[cursor..cursor + card_width].iter().map(|line| {
            line.split_whitespace().map(|n| Some(n.parse().unwrap())).collect()
        }).collect())
    }

    fn check_bingo(card_width: usize, card: &Vec<Vec<Option<i32>>>) -> bool {
        (0..card_width).any(|i| (0..card_width).all(|j| card[i][j].is_none())) ||
            (0..card_width).any(|i| (0..card_width).all(|j| card[j][i].is_none()))
            // (0..card_width).all(|i| card[i][i].is_none()) ||
            // (0..card_width).all(|i| card[i][card_width - i - 1].is_none())
    }

    fn mark_number(number: i32, card: &mut Vec<Vec<Option<i32>>>) {
        for line in card {
            for cell in line {
                if cell.filter(|n| *n == number).is_some() {
                    cell.take();
                }
            }
        }
    }

    let mut first_win_res: Option<i32> = None;

    let mut call_iter = calls.iter();

    'call_loop: for call in call_iter.by_ref() {
        for mut card in cards.iter_mut() {
            mark_number(*call, &mut card);
        }
        for card in cards.iter() {
            if check_bingo(card_width, card) {
                first_win_res = Some(card.iter().map(|line|
                    line.iter().map(|cell| cell.unwrap_or(0)).sum::<i32>())
                    .sum::<i32>() * call);
                break 'call_loop
            }
        }
    }

    let mut last_win_res: Option<i32> = None;

    'call_loop: for call in call_iter {
        for mut card in cards.iter_mut() {
            mark_number(*call, &mut card);
        }
        if cards.len() > 1 {
            cards = cards.into_iter().filter(|card| !check_bingo(card_width, card)).collect();
        } else if cards.len() == 1 && check_bingo(card_width, &cards[0]) {
            last_win_res = Some(&cards[0].iter().map(|line|
                line.iter().map(|cell| cell.unwrap_or(0)).sum::<i32>())
                .sum::<i32>() * call);
            break 'call_loop
        } else if cards.len() == 0 {
            return Err("Multiple cards called simultanous last bingo".into());
        }
    }

    if first_win_res.is_none() {
        return Err("No bingo called".into())
    }

    Ok([Some(first_win_res.unwrap().to_string()), Some(last_win_res.unwrap().to_string())])
}


#[test]
pub fn test_day4() {
    assert!(common::run_test(DAY, &run))
}