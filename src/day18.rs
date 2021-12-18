use std::rc::{Rc, Weak};
use std::str::Chars;
use aoc_runner_derive::*;

enum Action {
    Explode,
    Split,
    AddRight(u8),
    AddLeft(u8)
}

struct Fishe {
    value: u8
}

impl Fishelike for Fishe {
    fn validate(&mut self, depth: u8) -> Vec<Action> {
        if self.value > 9 {
            return vec![Action::Explode]
        }
        vec![]
    }
}

struct Fishpair {
    left: Box<dyn Fishelike>,
    right: Box<dyn Fishelike>
}


impl Fishelike for Fishpair {
    fn validate(&mut self, depth: u8) -> Vec<Action> {
        if depth == 4 {
            return vec!(Action::Split);
        }
        let mut return_actions = vec![];
        let left_actions = self.left.validate(depth+1);
        for a in left_actions {
            match a {
                Action::Explode => {
                    
                }
                Action::Split => {}
                Action::AddRight(_) => {}
                Action::AddLeft(_) => {}
            }
        }

        return_actions
    }
}

impl Fishpair {
    fn from_inp(mut inp: &mut Chars) -> Box<dyn Fishelike> {
        let mut new: Box<dyn Fishelike>;
        match inp.next().unwrap() {
            '[' => {
                let left = Fishpair::from_inp(inp);
                inp.next();   // This is the separating comma
                let right = Fishpair::from_inp(inp);
                new = Box::new(Fishpair{ left, right });
                inp.next(); // Closing bracket
            },
            num => {
                new = Box::new(Fishe{ value: (num as u8) - ('0' as u8) })
            }
        }

        new
    }


    fn add(first: Box<dyn Fishelike>, second: Box<dyn Fishelike>) -> Box<Fishpair> {
        let mut new = Box::new(Fishpair { left: first, right:  second });
        new.validate(0);
        new
    }
}

trait Fishelike {
    fn validate(&mut self, depth: u8) -> Vec<Action>;

}


#[aoc(day18, part1)]
fn sol1(inp: &str) -> u64{
    let mut pairs = inp.lines().map(|line| {
        Fishpair::from_inp(&mut line.chars())
    });

    let mut base = pairs.next().unwrap();
    for p in pairs {
        base = Fishpair::add(base, p);
    }

    println!("done");

    42
}