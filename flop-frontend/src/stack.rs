use core::fmt;
use std::{collections::VecDeque, fmt::Display};

#[derive(Debug, PartialEq, Clone)]
pub struct Stack<K> {
    pub data: VecDeque<K>,
}

impl<K> Stack<K> {
    pub fn new() -> Self {
        Self {
            data: Vec::new().into(),
        }
    }

    pub fn push(&mut self, token: K) {
        self.data.push_back(token);
    }

    pub fn pop(&mut self) -> Option<K> {
        self.data.pop_back()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn last(&self) -> Option<&K> {
        self.data.back()
    }

    pub fn pop_front(&mut self) -> Option<K> {
        self.data.pop_front()
    }
}

impl<K: fmt::Display> fmt::Display for Stack<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut elements = self
            .data
            .iter()
            .map(|k| k.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        if elements.is_empty() {
            elements = String::from("Empty Stack");
        }
        write!(f, "[{}]", elements)
    }
}
