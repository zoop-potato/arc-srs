#![allow(unused)]

use core::fmt;
use std::{sync::Arc, ops::Deref};

fn main() {
    let oe = SRS::new_text(&"oe");
    let second_step = oe.wrap();
    let third_step = SRS::new_list(&[second_step.clone(), oe.clone()]);
    let forth_step = third_step.wrap(); // this is the first iteration of the rule
    let fifth_step = SRS::new_list(&[third_step.clone(), oe.clone()]).wrap();
    let it = SRS::new_list(&[fifth_step.clone(), forth_step.clone(), third_step.clone()]).wrap();
    //println!("{oe:?}");
    let mut iter = forth_step.clone().into_iter();
    loop {
        let c = iter.next();
        if c.is_none() {
            break;
        }
        let c = c.unwrap();
        if c.eq(&'e') {
            let wrap = iter.wrap_of_stack_index(iter.get_top_stack_index().unwrap()).unwrap().0;
            println!("{}", wrap);
        }
    }
    //println!("\n{:?}", it);
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SRS {
    Text(Arc<str>),
    List(Arc<[SRS]>),
}

impl SRS {
    fn wrap(&self) -> Self {
        let (before, after) = (SRS::new_text(&"("), SRS::new_text(&")"));
        let wrapped = SRS::new_list(&[before, self.clone(), after]);
        return wrapped;
    }

    fn new_text(s: &str) -> Self {
        SRS::Text { 0: s.into() }
    }

    fn new_list(list: &[SRS]) -> Self {
        let collection = list.iter().map(|x| x.clone()).collect::<Vec<SRS>>();
        SRS::List {
            0: collection.into(),
        }
    }

    fn is_wrap(&self) -> bool {
        match self {
            SRS::Text(_) => return false,
            SRS::List(inner) => {
                if inner.len() != 3 {
                    return false;
                }
                match &inner[0] {
                    SRS::List(_) => return false,
                    SRS::Text(before) => {
                        if !before.deref().eq("(") {
                            return false;
                        }
                    },
                }
                match &inner[2] {
                    SRS::List(_) => return false,
                    SRS::Text(after) => {
                        if !after.deref().eq(")") {
                            return false;
                        }
                    },
                }
                return true;
            },
        }
    }
}

impl IntoIterator for SRS {
    type Item = char;
    type IntoIter = SrsIter;

    fn into_iter(self) -> Self::IntoIter {
        SrsIter {
            stack: vec![(self, 0)],
        }
    }
}

impl fmt::Display for SRS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string: String = self.clone().into_iter().collect();
        write!(f, "{}", string)
    }
}

#[derive(Debug)]
pub struct SrsIter {
    stack: Vec<(SRS, usize)>,
}

impl SrsIter {
    // Don't call on empty stack
    fn push_until_text(&mut self) {
        let last = self
            .stack
            .last()
            .expect("Called push_until_text on empty stack");
        match &last.0 {
            SRS::Text(_) => return,
            SRS::List(inner) => {
                self.stack.push((inner[last.1].clone(), 0));
                self.push_until_text()
            }
        };
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    // returns None if the top of the stack is not the Text variant
    // or the stack is empty
    // or the last index is out of bounds
    fn get_char(&self) -> Option<char> {
        let last = self.stack.last()?;
        match &last.0 {
            SRS::Text(string) => return string.chars().nth(last.1),
            SRS::List(_) => return None,
        }
    }

    fn increment_at_top(&mut self) {
        let last_indx = self.stack.len();
        if last_indx > 0 {
            let last_index = last_indx -1;
            let mut incr = self.stack.get_mut(last_index).unwrap();
            incr.1 += 1;
        }
    }

    fn top_index_valid(&self) -> bool {
        let last = self.stack.last();
        if last.is_none() {
            return false;
        };
        let (srs, indx) = last.unwrap();
        match srs {
            SRS::Text(string) => return string.len() > *indx,
            SRS::List(list) => return list.len() > *indx,
        }
    }

    /// The returned usize is the index on the stack of the returned SRS,
    /// not the index stored with the SRS 
    pub fn wrap_of_stack_index(&self, mut stack_index: usize) -> Option<(SRS, usize)> {
        if stack_index >= self.stack.len() {
            return None;
        }
        loop {
            if stack_index < 1 {
                return None;
            }
            stack_index -= 1;
            let srs = &self.stack[stack_index].0;
            if srs.is_wrap() {
                return Some((srs.clone(), stack_index));
            }
        }
    }

    pub fn get_top_stack_index(&self) -> Option<usize> {
        let len = self.stack.len();
        if len > 0 {
            return Some(len - 1);
        } else {
            return None;
        }
    }
}

impl Iterator for SrsIter {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            return None;
        }

        if self.top_index_valid() {
            self.push_until_text();
            let c = Some(self.get_char().expect("Got None while text index is valid"));
            self.increment_at_top();
            return c;
        } else {
            self.pop();
            self.increment_at_top();
            return self.next();
        }
    }
}
