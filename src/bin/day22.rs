use std::collections::{HashMap, HashSet};
use std::slice;

use itertools::Itertools;

use aoc2021_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "day22";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

type Cuboid = [[i32; 3]; 2];


pub fn run(input: &Vec<String>) -> AOCResult {
    let mut reactor = [[[false; 101]; 101]; 101];
    let mut init_cuboids = Vec::with_capacity(20);
    let mut cuboids = Vec::with_capacity(input.len());
    for line in input {
        let (on, s) = line.split(" ").next_tuple().unwrap();
        let on = on == "on";
        let ((lx, ux), (ly, uy), (lz, uz)) = s.split(",").map(|ax| ax[2..]
            .split("..")
            .map(str::parse::<i32>)
            .map(Result::unwrap)
            .next_tuple().unwrap()
        ).next_tuple().unwrap();
        if (lx >= -50 && ux <= 50 && ly >= -50 && uy <= 50 && lz >= -50 && uz <= 50) {
            for ((x, y), z) in ((lx + 50) as usize..(ux + 51) as usize)
                .cartesian_product((ly + 50) as usize..(uy + 51) as usize)
                .cartesian_product((lz + 50) as usize..(uz + 51) as usize) {
                reactor[z][y][x] = on;
            }
            init_cuboids.push((on, [[lx, ly, lz], [ux + 1, uy + 1, uz + 1]]))
        }
        cuboids.push((on, [[lx, ly, lz], [ux + 1, uy + 1, uz + 1]]));
    }
    let res1 = reactor.iter().flat_map(|x| x.iter().flat_map(|x| x.iter()))
        .filter(|c| **c)
        .count();
    assert_eq!(Some(res1.to_string()), run2(init_cuboids));
    Ok([Some(res1.to_string()), run2(cuboids)])
}


fn run2(cuboids: Vec<(bool, Cuboid)>) -> Option<String> {
    let mut grids = [Vec::new(), Vec::new(), Vec::new()];
    for (_, c) in cuboids.iter() {
        for p in *c {
            for i in 0..3 {
                grids[i].push(p[i]);
            }
        }
    }

    grids[0].sort();
    grids[1].sort();
    grids[2].sort();

    let mut grid_map = [HashMap::new(), HashMap::new(), HashMap::new()];
    for i in 0..3 {
        let mut virtual_grid_line = 0usize;
        for grid_line in grids[i].clone().into_iter().unique() {
            grid_map[i].insert(grid_line, virtual_grid_line);
            grids[i][virtual_grid_line] = grid_line;
            virtual_grid_line += 1;
        }
    }

    let (x_len, y_len, z_len) = grid_map.iter().map(HashMap::len).next_tuple().unwrap();
    println!("{} {} {}", x_len, y_len, z_len);

    let mut reactor = vec![false; z_len * y_len * x_len];

    let index = |x, y, z| z * x_len * y_len + y * x_len + x;

    for (on, [[lx, ly, lz], [ux, uy, uz]]) in cuboids.iter() {
        for z in *grid_map[2].get(lz).unwrap()..*grid_map[2].get(uz).unwrap() {
            for y in *grid_map[1].get(ly).unwrap()..*grid_map[1].get(uy).unwrap() {
                for x in *grid_map[0].get(lx).unwrap()..*grid_map[0].get(ux).unwrap() {
                    reactor[index(x, y, z)] = *on;
                }
            }
        }
    }

    let mut res = 0;

    for z in 0..z_len {
        for y in 0..y_len {
            for x in 0..x_len {
                if reactor[index(x, y, z)] {
                    res += ((grids[2][z + 1] - grids[2][z]) as u64)
                        * ((grids[1][y + 1] - grids[1][y]) as u64)
                        * ((grids[0][x + 1] - grids[0][x]) as u64)
                }
            }
        }
    }

    Some(res.to_string())
}

#[test]
pub fn test_day22() {
    assert!(common::run_test(DAY, &run))
}