#![allow(arithmetic_overflow)]
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

    fn get_p1_coefficient(&self) -> i64 {
        match self {
            BracketType::Round => {3}
            BracketType::Square => {57}
            BracketType::Curly => {1197}
            BracketType::Sharp => {25137}
        }
    }
}

pub fn run(inp: &str) -> i64 {
    inp.as_bytes().split(|char| char == &('\n' as u8)).map(|line| {
        let mut stack_index: usize = 0-1;
        let mut stack: MaybeUninit<[BracketType; 110]> = MaybeUninit::uninit();
        // let mut stack: [BracketType; 110] = [BracketType::Sharp; 110];

        let corrupted: Option<BracketType> = line.iter().find_map(|b| unsafe {
            let (bracket, opening) = BracketType::from_byte(b);
            if opening {
                stack_index += 1;
                *stack.assume_init_mut().get_unchecked_mut(stack_index) = bracket;
                None
            } else {
                if *stack.assume_init().get_unchecked(stack_index) != bracket {
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
            0i64
        }
    }).sum()

}