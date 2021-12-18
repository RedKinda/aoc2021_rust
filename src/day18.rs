use std::rc::{Rc, Weak};
use std::str::Chars;
use aoc_runner_derive::*;

enum Action {
    Explode,
    Split,
    AddRight(u8),
    AddLeft(u8)
}

enum FishNode {
    Pair(Fishpair),
    Literal(Fishe)
}

impl FishNode {
    fn validate(&mut self, depth: u8) -> Vec<Action> {
        if depth == 4 {
            return vec!(Action::Split);
        }
        let mut return_actions = vec![];

        match self {
            FishNode::Pair(pair) => {
                let left_actions = pair.left.validate(depth+1);
                for a in left_actions {
                    match a {
                        Action::Explode => {

                        }
                        Action::Split => {}
                        Action::AddRight(_) => {}
                        Action::AddLeft(_) => {}
                    }
                }
            }
            FishNode::Literal(val) => {}
        }



        return_actions
    }

}

struct Fishe {
    value: u8
}


struct Fishpair {
    left: Box<FishNode>,
    right: Box<FishNode>
}


impl Fishpair {

    fn from_inp(mut inp: &mut Chars) -> Box<FishNode> {
        let mut new: Box<FishNode>;
        match inp.next().unwrap() {
            '[' => {
                let left = Fishpair::from_inp(inp);
                inp.next();   // This is the separating comma
                let right = Fishpair::from_inp(inp);
                new = Box::new(FishNode::Pair(Fishpair{ left, right }));
                inp.next(); // Closing bracket
            },
            num => {
                new = Box::new(FishNode::Literal(Fishe{ value: (num as u8) - ('0' as u8) }))
            }
        }

        new
    }


    fn add(first: Box<FishNode>, second: Box<FishNode>) -> FishNode {
        let mut new = FishNode::Pair(Fishpair { left: first, right:  second });
        new.validate(0);
        new
    }
}


#[aoc(day18, part1)]
fn sol1(inp: &str) -> u64{
    let mut pairs = inp.lines().map(|line| {
        Fishpair::from_inp(&mut line.chars())
    });

    let mut base = pairs.next().unwrap();
    for p in pairs {
        base = Box::from(Fishpair::add(base, p));
    }

    println!("done");

    42
}