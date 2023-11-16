use std::collections::VecDeque;

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
