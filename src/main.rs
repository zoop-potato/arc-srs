#![allow(unused)]

use std::sync::Arc;

fn main() {
    let oe = SRS::new_text(&"oe").wrap().wrap();
    //println!("{oe:?}");
    for c in oe {
        print!("{}", c);
    }
}

#[derive(Clone, Debug)]
enum SRS {
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

#[derive(Debug)]
struct SrsIter {
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
