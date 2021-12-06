
use aoc_runner_derive::*;
use std::sync::Arc;


#[aoc(day4, part1)]
pub fn solve4_part1(inp: &str) -> u32 {
    run(&inp, u8::lt, 100)
}

#[aoc(day4, part2)]
pub fn solve4_part2(inp: &str) -> u32 {
    run(&inp, u8::gt, 0)
}


const START_VAL: u8 = 0;


pub fn run(inp: &str, cmp_fn: fn(&u8, &u8) -> bool, initial: u8) -> u32 {
    let mut original: [u8; 100]  = [0; 100];
    let mut move_order: [u8; 100] = [0; 100];
    let mut lines = inp.lines();


    (original, move_order) = lines.next().unwrap().split(',').enumerate().fold((original, move_order), |(mut original, mut order), (index, val)| {
        let n = val.parse::<usize>().unwrap();
        order[n] = index as u8;
        original[index] = n as u8;
        (original, order)
    });

    let mut best_grid: [[u8; 5]; 5] = [[0; 5]; 5];
    let mut best_grid_time: u8 = initial;


    for i in 0..100 {
        let mut min_row = START_VAL;
        let mut min_colse: [u8; 5] = [START_VAL; 5];
        let mut grid: [[u8; 5]; 5] = [[START_VAL; 5]; 5];
        let mut grid_time: u8 = 100;

        lines.next();
        for line in 0..5 {
            min_row = START_VAL;
            (lines.next().unwrap().to_owned() + "\n").as_bytes().chunks_exact(3).enumerate().map(|(ind, chunk)| {
                let mut num = chunk[1] - '0' as u8;
                if chunk[0] != ' ' as u8 && chunk[0] != '\n' as u8 {
                    num += 10 * (chunk[0] as u8 - ('0' as u8));
                }
                num = move_order[num as usize];

                min_colse[ind] = {
                    if num > min_colse[ind] {
                        num
                    } else {
                        min_colse[ind]
                    }
                };

                min_row = {
                    if num > min_row {
                        num
                    } else {
                        min_row
                    }
                };
                grid[line][ind] = num;
            }).collect::<()>();

            if min_row < grid_time {
                grid_time = min_row;
            }
        }

        for i in 0..5 {
            if min_colse[i] < grid_time {
                grid_time = min_colse[i]
            }
        }

        if cmp_fn(&grid_time, &best_grid_time) {
            best_grid_time = grid_time;
            best_grid = grid;
        }
    }

    let sum: u32 = best_grid.iter().flatten().map(|n| {
        if n > &best_grid_time {
            original[*n as usize] as u32
        } else {
            0
        }
    }).sum();

    (original[best_grid_time as usize] as u32) * sum
}

