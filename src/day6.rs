use aoc_runner_derive::*;

/*
#[aoc(day6, part1)]
pub fn solve6_part1(inp: &str) -> u64 {
    run(&inp, 80)
}

#[aoc(day6, part2)]
pub fn solve6_part2(inp: &str) -> u64 {
    run(&inp, 256)
}

 */

#[aoc(day6, part1)]
pub fn solve6_part1(inp: &[u64; 9]) -> u64 {
    run(inp.to_owned(), 80)
}

#[aoc(day6, part2)]
pub fn solve6_part2(inp: &[u64; 9]) -> u64 {
    run(inp.to_owned(), 256)
}

#[aoc_generator(day3)]
pub fn input_generator3(inp: &str) -> [u64; 9] {
    inp.split(',').fold([0; 9], |mut res, num| {
        unsafe {
            *res.get_unchecked_mut((*num.as_bytes().get_unchecked(0) as usize) - '0' as usize) += 1;
            res
        }
    })
}

fn run(mut ages: [u64; 9], length: u16) -> u64 {
    let weeks = length / 7;

    for _ in 0..weeks {
        // println!("{:?}, sum: {}", ages, ages.iter().sum::<u64>());
        let sixth = ages[6].clone();
        let fifth = ages[5].clone();


        ages[6] += ages[4];
        ages[5] += ages[3];
        ages[4] += ages[2];

        ages[3] += ages[1];
        ages[2] += ages[0];

        ages[0] += ages[7];
        ages[1] += ages[8];

        ages[8] = sixth;
        ages[7] = fifth;

    }

    ages.iter().sum::<u64>() + ages.into_iter().take(((length) % 7) as usize).sum::<u64>()
}
