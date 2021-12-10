use aoc_runner_derive::*;
use std::mem::MaybeUninit;

const PART1: bool = true;
const ROUND_OPEN: u8 = '(' as u8;
const ROUND_CLOSE: u8 = ')' as u8;
const SQUARE_OPEN: u8 = '[' as u8;
const SQUARE_CLOSE: u8 = ']' as u8;
const CURLY_OPEN: u8 = '{' as u8;
const CURLY_CLOSE: u8 = '}' as u8;
const SHARP_OPEN: u8 = '<' as u8;
const SHARP_CLOSE: u8 = '>' as u8;

#[derive(PartialEq, Copy, Clone)]
enum BracketType {
    Round,
    Square,
    Curly,
    Sharp
}

impl BracketType {
    fn from_byte(byte: &u8) -> (BracketType, bool){
        match byte {
            &ROUND_OPEN => (BracketType::Round, true),
            &ROUND_CLOSE => (BracketType::Round, false),
            &SQUARE_OPEN => (BracketType::Square, true),
            &SQUARE_CLOSE => (BracketType::Square, false),
            &CURLY_OPEN => (BracketType::Curly, true),
            &CURLY_CLOSE => (BracketType::Curly, false),
            &SHARP_OPEN => (BracketType::Sharp, true),
            &SHARP_CLOSE => (BracketType::Sharp, false),
            _ => {panic!("Invalid character")}
        }
    }

    fn get_p1_coefficient(&self) -> u32 {
        match self {
            BracketType::Round => {3}
            BracketType::Square => {57}
            BracketType::Curly => {1197}
            BracketType::Sharp => {25137}
        }
    }
}

#[aoc(day10, part1)]
pub fn solve_10(inp: &str) -> u32 {
    inp.as_bytes().split(|char| char == &('\n' as u8)).map(|line| {
        let mut stack_index: usize = 0;
        // let mut stack: MaybeUninit<[BracketType; 110]> = unsafe {MaybeUninit::uninit().assume_init()};
        let mut stack: [BracketType; 110] = [BracketType::Sharp; 110];

        let corrupted: Option<BracketType> = line.iter().find_map(|b| {
            let (bracket, opening) = BracketType::from_byte(b);
            if opening {
                stack[stack_index] = bracket;
                stack_index += 1;
                None
            } else {
                if stack[stack_index-1] != bracket {
                    Some(bracket)
                } else {
                    stack_index -= 1;
                    None
                }
            }
        });

        if let Some(b) = corrupted {
            b.get_p1_coefficient()
        } else {
            0
        }
    }).sum()

}