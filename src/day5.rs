use aoc_runner_derive::*;


#[aoc(day5, part1)]
pub fn solve5_part1(inp: &str) -> u32 {
    run(&inp, false)
}

#[aoc(day5, part2)]
pub fn solve5_part2(inp: &str) -> u32 {
    run(&inp, true)
}


pub fn add_bit(field_one: &mut [[u128; 8]; 1000], field_crossing: &mut [[u128; 8]; 1000], row: usize, col: u8) -> bool{
    let index_col: usize = (col / 128) as usize;
    let bit_col = col % 128;
    if (field_crossing[row][index_col]) & (1 << bit_col) == 0 {
        if field_one[row][index_col] & (1 << bit_col) == 1 {
            field_crossing[row][index_col] += (1 << bit_col);
            return true
        } else {
            field_one[row][index_col] += (1 << bit_col);
        }
    }
    false
}


pub fn run(inp: &str, diagonals: bool) -> u32 {
    let mut field_one: [[u128; 8]; 1000] = [[0; 8]; 1000];
    let mut field_crossing: [[u128; 8]; 1000] = [[0; 8]; 1000];
    let mut final_count: u32 = 0;

    let a = inp.as_bytes().split(|v| v == &('\n' as u8)).map(|line| {
        let coords: [u8; 4] = [0; 4];
        let mut ind = 0;
        let mut current: u8 = 0;
        let coords = line.iter().fold(coords, |mut current, val| {
            if *val >= '0' as u8 && *val <= '9' as u8 {
                current[ind] = current[ind] * 10 + val;
            }
            current
        });

        if coords[0] != coords[2] && coords[1] != coords[3] {  // This means this is a diagonal
            if diagonals {

            }
        }
        else {
            if coords[0] == coords[2] {  // Line is horizontal, efficient adding
                let mut line_len = (coords[0]).abs_diff(coords[2]) + 1;
                let lower_x = std::cmp::min(coords[0], coords[2]);

                let row = coords[1] as usize;
                let index_col = (lower_x / 128) as usize;
                let index_bit = lower_x % 128;

                let mask = !0u128 >> index_bit;
                let add_to_crossing = (mask & field_one[row][index_col]) & !field_crossing[row][index_col];
                field_crossing[row][index_col] += add_to_crossing;

                for i in 0..128 {
                    if add_to_crossing & (1 << i) {
                        final_count += 1
                    }
                }

                field_crossing[row][index_col] += add_to_crossing;



            } else {  // Line is vertical, cringe adding
                let line_len = (coords[1]).abs_diff(coords[3]) + 1;
                let lower = std::cmp::min(coords[1], coords[3]);
                for row in 0..line_len {
                    let col = coords[0] % 128;
                    if add_bit(&mut field_one, &mut field_crossing, (lower + row) as usize, col) {
                        final_count += 1;
                    }
                }
            }
        }

    });

    0
}