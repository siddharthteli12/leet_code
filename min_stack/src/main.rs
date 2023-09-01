use std::collections::{HashMap, HashSet};

struct MinStack {
    list: Vec<i32>,
    min_list: Vec<i32>,
}
impl MinStack {
    fn new() -> Self {
        Self {
            list: vec![],
            min_list: vec![],
        }
    }

    fn push(&mut self, val: i32) {
        self.list.push(val);
        if self.min_list.is_empty() || self.min_list.last().unwrap() >= &val {
            self.min_list.push(val);
        }
    }

    fn pop(&mut self) {
        let value = self.list.pop().unwrap();
        if self.min_list.last().unwrap() == &value {
            self.min_list.pop();
        }
    }

    fn top(&self) -> i32 {
        *self.list.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min_list.last().unwrap()
    }
}

fn main() {}
