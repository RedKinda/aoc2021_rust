const LENGTH: u16 = 256;


pub fn run(inp: &str) -> i64 {
    let weeks = LENGTH / 7;

    let mut ages: [u64; 9] = inp.as_bytes().chunks_exact(2).fold([0; 9], |mut res, num| {
        unsafe {
            *res.get_unchecked_mut((*num.as_bytes().get_unchecked(0) as usize) - '0' as usize) += 1;
            res

        }
    });

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

    let remainder = ages.into_iter().take(((LENGTH) % 7) as usize).sum::<u64>();
    (ages.iter().sum::<u64>() + remainder) as i64
}
