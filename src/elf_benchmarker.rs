
pub fn run(inp: &str) -> i64{
    solve_part1(&input_generator(inp)) as i64
}



pub fn input_generator(input: &str) -> Vec<u16> {
     input
        .lines()
        .map(|l| {
            l.chars().into_iter().enumerate().map(|(ind, c)| {
                if c == '1' {
                    (1 << 11-ind)
                } else {
                    0
                }

            }).sum()
        }).collect()
}


pub fn solve_part1(input: &Vec<u16>) -> u32 {
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
