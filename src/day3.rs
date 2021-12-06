use aoc_runner_derive::*;

#[aoc_generator(day3)]
pub fn input_generator3(input: &str) -> Vec<u16> {
    input
        .lines().into_iter()
        .map(|l| {
            l.chars().into_iter().enumerate().map(|(ind, c)| {
                if c == '1' {
                    (1 << 11 - ind)
                } else {
                    0
                }
            }).sum::<u16>()
        }).collect()
}


#[aoc(day3, part1)]
pub fn solve3_part1(input: &Vec<u16>) -> u32 {
    let mut res: u32 = 0;
    let mut res2: u32 = 0;
    for i in [11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0] {
        let mut ones: u16 = input.iter().map(|n| {
            if n & (1 << i) > 0 {
                1
            } else {
                0
            }
        }).sum();

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



/*
#[aoc_generator(day3)]
pub fn input_generator3(input: &str) -> [u8; 8192] {
    let mut res: [u8; 8192] = [0; 8192];
    let nums: Vec<()> = input
        .lines().into_iter()
        .map(|l| {
            l.chars().into_iter().enumerate().map(|(ind, c)| {
                if c == '1' {
                    (1 << 11 - ind)
                } else {
                    0
                }
            }).sum::<usize>()
        }).into_iter().map(|n| {
        res[n] += 1;
    }).collect();

    res
}


#[aoc(day3, part2)]
pub fn solve3_part2(input: &[u8; 8192]) -> u32 {
    let mut prefix_sums: [u32; 8192] = [0; 8192];
    let mut prev: u32 = 0;
    for (ind, n) in input.into_iter().enumerate() {
        prefix_sums[ind] = (*n as u32 + prev);
        prev += *n as u32;
    }

    let mut low = 0;
    let mut high = 8192;

    while low + 1 < high {
        let mid = (high + low) / 2;
        if prefix_sums[mid] - prefix_sums[low] >= (prefix_sums[high-1]  - prefix_sums[low]) / 2 {
            high = mid;
        } else {
            low = mid;
        }
    }

    let ans1 = low.clone() + 1;
    low = 0;
    high = 8192;

    while low + 1 < high {
        let mid = (high + low) / 2;
         if prefix_sums[mid] - prefix_sums[low] < (prefix_sums[high-1]  - prefix_sums[low]) / 2 {
            high = mid;
        } else {
            low = mid;
        }
    }


    ((low+1) * ans1) as u32
}

 */