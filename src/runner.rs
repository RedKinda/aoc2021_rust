mod elf_benchmarker;


// use crate::*;
use std::fs;

fn main() {
    let inp = fs::read_to_string("input/2021/day3.txt").expect("Couldn't read file");
    let res = elf_benchmarker::run(&*inp);
    println!("Result: {}", res);
}


