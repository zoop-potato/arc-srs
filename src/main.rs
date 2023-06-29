#![allow(unused)]

use core::fmt;
use std::{ops::Deref, sync::Arc};
use lazy_static::lazy_static;

const EXPAND: char = 'e';
const OUTREACH: char = 'o';
lazy_static!{
    static ref OPEN: SRS = SRS::Text("(".into());
    static ref CLOSE: SRS = SRS::Text(")".into());
}

fn main() {
    let oe = SRS::new_text("oe").wrap().wrap();
    println!("{}", oe);
    println!("---------------------------------------------");
    let expand_1 = oe.expand();
    println!("{}", expand_1);
    println!("---------------------------------------------");
    let expand_2 = expand_1.expand();
    println!("{}", expand_2);
    println!("---------------------------------------------");
    let expand_3 = expand_2.expand();
    println!("{}", expand_3);
    println!("---------------------------------------------");
    let expand_4 = expand_3.expand();
    println!("{}", expand_4);
    println!("---------------------------------------------");
    let expand_5 = expand_4.expand();
    println!("{}", expand_5);
    //println!("---------------------------------------------");
    //println!("Expand 5 is {} long.", expand_5.to_string().len());
    //let big = expand_5.expand();
    //println!("Done building Big!");
    //println!("Building Extra Big...");
    //let big2 = big.expand();
    //println!("Done Building Extra Big!");
    //println!("Counting Extra Big's length...");
    /*let mut counter: u128 = 0;
    for _ in big {
        counter += 1;
        if counter == u128::MAX {panic!("Max Reached")}
    }*/
    //println!("Big is {} long", counter);
    //
    //println!("Done!");
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SRS {
    Text(Arc<str>),
    List(Arc<[SRS]>),
}

impl SRS {
    fn wrap(&self) -> Self {
        let (before, after) = (OPEN.clone(), CLOSE.clone());
        let wrapped = SRS::new_list(&[before, self.clone(), after]);
        return wrapped;
    }

    fn new_text(s: &str) -> Self {
        SRS::Text(s.into())
    }

    fn new_list(list: &[SRS]) -> Self {
        let collection = list.to_vec();
        SRS::List(collection.into())
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
                    }
                }
                match &inner[2] {
                    SRS::List(_) => return false,
                    SRS::Text(after) => {
                        if !after.deref().eq(")") {
                            return false;
                        }
                    }
                }
                return true;
            }
        }
    }

    fn expand(&self) -> SRS {
        let mut iter = self.clone().into_iter();
        let mut wrap_stack: Vec<Vec<SRS>> = vec![vec![]];
        let mut temp_string = "".to_string();
        loop {
            let c = iter.next();
            if c.is_none() {
                let bottom_layer = wrap_stack.pop().unwrap();
                return SRS::new_list(&bottom_layer);
            }
            let c = c.unwrap();

            
            match c {
                '(' => {
                    if !temp_string.is_empty() {
                        let push = SRS::new_text(&temp_string);
                        wrap_stack.last_mut().unwrap().push(push);
                        temp_string.clear();
                    } // temp_string cleared
                    wrap_stack.push(vec![]);
                }
                ')' => {
                    if !temp_string.is_empty() {
                        let push = SRS::new_text(&temp_string);
                        wrap_stack.last_mut().unwrap().push(push);
                        temp_string.clear();
                    } // temp_string cleared
                    let layer = wrap_stack.pop().unwrap();
                    let consolidated = SRS::new_list(&layer[..]);
                    wrap_stack.last_mut().unwrap().push(consolidated.wrap());
                }
                EXPAND => {
                    if !temp_string.is_empty() {
                        let push = SRS::new_text(&temp_string);
                        wrap_stack.last_mut().unwrap().push(push);
                        temp_string.clear();
                    } // temp_string cleared
                    let top_index = iter.index_of_top_of_stack().unwrap();
                    let wrap = iter.wrap_of_stack_index(top_index).unwrap().0;
                    // Get the middle element of the wrap
                    let expansion = match &wrap {
                        SRS::List(list) => &list.deref()[1],
                        SRS::Text(_) => panic!("Wrap should not be the SRS::Text"),
                    }
                    .clone();
                    wrap_stack.last_mut().unwrap().push(expansion);
                }
                OUTREACH => {
                    if !temp_string.is_empty() {
                        let push = SRS::new_text(&temp_string);
                        wrap_stack.last_mut().unwrap().push(push);
                        temp_string.clear();
                    } // temp_string cleared
                    let top_index = iter.index_of_top_of_stack().unwrap();
                    let wrap = iter.wrap_of_stack_index(top_index).unwrap().1;
                    let wrap_of_wrap = iter.wrap_of_stack_index(wrap).unwrap().0;
                    // Get the middle element of the wrap
                    let expansion = match &wrap_of_wrap {
                        SRS::List(list) => &list.deref()[1],
                        SRS::Text(_) => panic!("Wrap should not be the SRS::Text"),
                    }
                    .clone();
                    wrap_stack.last_mut().unwrap().push(expansion);
                }
                _ => temp_string.push(c),
            }
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
            let last_index = last_indx - 1;
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

    pub fn index_of_top_of_stack(&self) -> Option<usize> {
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
