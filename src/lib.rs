

use aoc_runner_derive::*;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<u16> {
     input
        .lines()
        .map(|l| {
            let mut ans: u16 = 0;
            let realans: u16 = l.chars().into_iter().enumerate().map(|(ind, c)| {
                if c == '1' {
                    // print!("{}", 1);  // (1 << 12-ind));
                    (1 << 11-ind)
                } else {
                    // print!("0");
                    0
                }

            }).sum();
            /*
            let mut res = 0;
            for b in 0..haha. {
                if b {
                    res = res * 2 + 1;
                } else {
                    res = res * 2;
                }
            }
            res

             */
            realans

        }).collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Vec<u16>) -> u32 {
    // println!("{}", input[999]);
    let mut res: u32 = 0;
    let mut res2: u32 = 0;
    for i in [11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0] {
        let mut ones: u16 = input.iter().map(|n| {
            if n & (1 << i) > 0 {
            //if (n >> i) % 2 == 1 {
                1
            } else {
                0
            }
        }).sum();

        /*for n in input {
            if (n >> i) % 2 == 1 {
                ones += 1;
            }
        }

         */

        if ones > (input.len() / 2) as u16 {
            res += 1;
        } else {
            res2 += 1;
        }


        res *= 2;
        res2 *= 2;
    }

    res * res2 / 4
}




aoc_lib!{ year = 2021 }