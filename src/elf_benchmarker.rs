const LENGTH: u16 = 256;
const WEEKS: usize = (LENGTH / 7) as usize;
const FISH_COUNT: u16 = 300;

pub fn run(inp: &str) -> i64 {
    let mut ages: [i64; 9] = [0; 9];

    inp.as_bytes().chunks_exact(2).for_each(|num| {
        unsafe {
            *ages.get_unchecked_mut((*num.get_unchecked(0) as usize) - '0' as usize) += 1;
        }
    });

    std::iter::repeat(()).take(WEEKS as usize).for_each(|()| {
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
    });

    ages.iter().sum::<i64>() + ages.into_iter().take(((LENGTH) % 7) as usize).sum::<i64>()
}
