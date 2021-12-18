use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Write};
use std::rc::{Rc, Weak};
use std::str::Chars;
use aoc_runner_derive::*;
use std::collections::{binary_heap, BinaryHeap};

enum Action {
    Explode,
    Split(Fishpair),
    AddRight(u8),
    AddLeft(u8)
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Explode => {
                f.write_str("Exploding")
            }
            Action::Split(pair) => {
                f.write_str(format!("Splitting into {}", pair).as_str())
            }
            Action::AddRight(val) => {
                f.write_str(format!("Adding {} to the right", val).as_str())
            }
            Action::AddLeft(val) => {
                f.write_str(format!("Adding {} to the left", val).as_str())
            }
        }
    }
}

impl Eq for Action {}

impl PartialEq<Self> for Action {
    fn eq(&self, other: &Self) -> bool {
        Ord::cmp(self, other) == Ordering::Equal
    }
}

impl PartialOrd<Self> for Action {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for Action {
    fn cmp(&self, other: &Self) -> Ordering {
        let my_ind = match self {
            Action::Explode => {1}
            Action::Split(_) => {0}
            Action::AddRight(_) => {3}
            Action::AddLeft(_) => {2}
        };
        let other_ind = match other {
            Action::Explode => {1}
            Action::Split(_) => {0}
            Action::AddRight(_) => {3}
            Action::AddLeft(_) => {2}
        };
        return my_ind.cmp(&other_ind)
    }
}

struct Fishe {
    value: u8
}

impl Fishelike for Fishe {
    fn validate(&mut self, depth: u8) -> BinaryHeap<Action> {
        if self.value > 9 {
            let left =  Box::new(Fishe{ value: (self.value / 2) });
            let right = Box::new(Fishe { value: (self.value / 2) + self.value % 2 });
            return BinaryHeap::from(vec!(Action::Split(Fishpair{left, right})));

        }
        BinaryHeap::from(vec!())
    }

    fn add_rightmost(&mut self, val: u8) {
        self.value += val;
    }

    fn add_leftmost(&mut self, val: u8) {
        self.value += val;
    }

    fn get_immediate_value(&self) -> u8 {
        self.value
    }

    fn get_magnitude(&self) -> u64 {
        self.value as u64
    }
}

impl Display for Fishe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*self.value.to_string())
    }
}

struct Fishpair {
    left: Box<dyn Fishelike>,
    right: Box<dyn Fishelike>
}

impl Display for Fishpair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("[");
        self.left.fmt(f)?;
        f.write_str(",");
        self.right.fmt(f)?;
        f.write_str("]");
        Ok(())
    }
}

impl Fishelike for Fishpair {
    fn validate(&mut self, depth: u8) -> BinaryHeap<Action> {

        let mut repeat_val = false;

        let mut return_actions = BinaryHeap::<Action>::new();

        loop {
            //println!("Diving left in: {}", self);
            let left_actions = self.left.validate(depth + 1);
            for action in left_actions {
                repeat_val = true;
                println!("Executing action '{}' on {} from {}", action, self.left, self);
                match action {
                    Action::Explode => {
                        self.left = Box::new(Fishe { value: 0 });
                    }
                    Action::Split(pair) => {
                        self.left = Box::new(pair);
                    }
                    Action::AddRight(val) => {
                        self.right.add_leftmost(val);
                    }
                    Action::AddLeft(val) => {
                        return_actions.push(Action::AddLeft(val));
                    }
                }
            }

            //println!("Diving right in: {}", self);
            let right_actions = self.right.validate(depth + 1);
            for action in right_actions {
                repeat_val = true;
                println!("Executing action '{}' on {} from {}", action, self.right, self);
                match action {
                    Action::Explode => {
                        self.right = Box::new(Fishe { value: 0 });
                    }
                    Action::Split(pair) => {
                        self.right = Box::new(pair);
                    }
                    Action::AddRight(val) => {
                        return_actions.push(Action::AddRight(val));
                    }
                    Action::AddLeft(val) => {
                        self.left.add_rightmost(val);
                    }
                }
            }
            if repeat_val {
                repeat_val = false;
            } else {
                break;
            }
        }

        if depth >= 4 {
            //println!("Exploding {}", self);
            let left = self.left.get_immediate_value();
            let right = self.right.get_immediate_value();

            vec!(Action::Explode, Action::AddLeft(left), Action::AddRight(right)).into_iter().for_each(|item| {
                return_actions.push(item);
            });

        }

        //println!("Validated: {}", self);

        return_actions
    }

    fn add_rightmost(&mut self, val: u8) {
        self.right.add_rightmost(val);
    }

    fn add_leftmost(&mut self, val: u8) {
        self.left.add_leftmost(val);
    }

    fn get_immediate_value(&self) -> u8 {
        println!("Error: Cant get immediate value for {}", self);
        panic!("Can't get immediate value, this is a pair")
    }

    fn get_magnitude(&self) -> u64 {
        self.left.get_magnitude() * 3 + self.right.get_magnitude() * 2
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
        Box::new(Fishpair { left: first, right:  second })
    }
}


trait Fishelike: Display {
    fn validate(&mut self, depth: u8) -> BinaryHeap<Action>;
    fn add_rightmost(&mut self, val: u8);
    fn add_leftmost(&mut self, val: u8);
    fn get_immediate_value(&self) -> u8;
    fn get_magnitude(&self) -> u64;
}


#[aoc(day18, part1)]
fn sol1(inp: &str) -> u64{
    let mut pairs = inp.lines().map(|line| {
        Fishpair::from_inp(&mut line.chars())
    });

    let mut base = pairs.next().unwrap();
    base.validate(0);
    println!("Current: {}", base);
    for mut p in pairs {
        p.validate(0);
        println!("Validated addition..");
        base = Fishpair::add(base, p);
        let remaining = base.validate(0);

        println!("Current: {} Actions left: {}", base, remaining.len());
    }


    base.get_magnitude()
}