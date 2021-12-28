use std::cmp;
use itertools::Itertools;
use aoc2021_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "day17";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

type CVec = [f32;2];

fn quad(a: f32,b: f32,c: f32, x: f32) -> f32 {
    a*x*x + b*x + c
}

fn abc(a: f32,b: f32,c: f32) -> Option<CVec> {
    let d: f32 = b*b-4f32*a*c;
    if d >= 0f32 {
        Some([(-b + d.sqrt())/(2f32*a), (-b - d.sqrt())/(2f32*a)])
    } else {
        None
    }
}

fn enter_t(v0: CVec, l_bounds: CVec) -> Option<f32> {
    let tx = abc(-0.5, v0[0] + 0.5, -l_bounds[0])?[0];
    let ty = abc(-0.5, v0[1] + 0.5, -l_bounds[1])?[1];
    Some(tx.max(ty))
}

fn exit_t(v0: CVec, u_bounds: CVec) -> Option<f32> {
    let tx = abc(-0.5, v0[0] + 0.5, -u_bounds[0]).map(|t| t[0]).unwrap_or(f32::MAX);
    let ty = abc(-0.5, v0[1] + 0.5, -u_bounds[1])?[1];
    Some(tx.min(ty))
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let (l_bound_x, u_bound_x, u_bound_y, l_bound_y): (i32, i32, i32, i32) = input[0].strip_prefix("target area: x=")
        .unwrap()
        .split(", y=")
        .flat_map(|s| s.split(".."))
        .map(str::parse)
        .map(Result::unwrap)
        .next_tuple().unwrap();

    let l_bounds = [l_bound_x as f32, l_bound_y as f32];
    let u_bounds = [u_bound_x as f32, u_bound_y as f32];

    let v_l_bounds = [
        abc(1f32,1f32, -2f32*l_bounds[0]).unwrap()[0].ceil() as i32,
        u_bounds[1] as i32,
    ];

    let v_u_bounds = [
        u_bounds[0] as i32,
        -u_bounds[1] as i32 -1,
    ];

    let res1 = cmp::max(
        quad(-0.5, v_u_bounds[1] as f32 + 0.5, 0f32, (v_u_bounds[1] as f32 + 0.5).floor()) as i32,
        quad(-0.5, v_u_bounds[1] as f32 + 0.5, 0f32, (v_u_bounds[1] as f32 + 0.5).ceil()) as i32,
    );

    let mut count = 0;

    for vy in v_l_bounds[1]..v_u_bounds[1]+1 {
        for vx in v_l_bounds[0]..v_u_bounds[0]+1 {
            if let Some(enter) = enter_t([vx as f32, vy as f32], l_bounds) {
                if let Some(exit) = exit_t([vx as f32, vy as f32], u_bounds) {
                    if (enter-0.0001).ceil() <= exit {
                        println!("{} {}: {} {}", vx, vy, enter, exit);
                        count += 1;
                    }
                }
            }

        }
    }

    Ok([Some(res1.to_string()), Some(count.to_string())])
}

#[test]
pub fn test_day17() {
    assert!(common::run_test(DAY, &run))
}