use aoc_runner_derive::*;

#[aoc_generator(day1)]
pub fn gen1(input: &str) -> Vec<u32> {
    input.lines().map(|line| {
        line.parse::<u32>().unwrap()
    }).collect::<Vec<u32>>()
}

#[aoc(day1, part1)]
pub fn solve1_part1(numbers: &Vec<u32>) -> u32 {
    let mut result: u32 = 0;
    let mut last_number: &u32 = &numbers[0];

    numbers.iter().for_each(|number| {
        if number > last_number {
            result += 1;
        }
        last_number = number;
    });

    result
}

#[aoc(day1, part2)]
pub fn solve1_part2(numbers: &Vec<u32>) -> u32 {
    let mut result: u32 = 0;
    let mut last_sum = numbers[0] + numbers[1] + numbers[2];

    for index in 0..numbers.len() - 2 {
        let sum = numbers[index] + numbers[index + 1] + numbers[index + 2];
        if sum > last_sum {
            result += 1;
        }
        last_sum = sum;
    }

    result
}