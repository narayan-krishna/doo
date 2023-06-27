use super::doolist::DooItem;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Serialize, Deserialize)]
pub struct CappedQueue<T> {
    pub items: VecDeque<T>,
    pub capacity: usize,
}

impl<T> CappedQueue<T> {
    pub fn new(capacity: usize) -> CappedQueue<T> {
        CappedQueue {
            items: VecDeque::new(),
            capacity,
        }
    }

    pub fn push_front(&mut self, item: T) -> Result<(), &'static str> {
        if self.items.len() < self.capacity {
            self.items.push_front(item);

            return Ok(());
        }

        return Err("Queue has reached max capacity, cannot push items to queue");
    }

    pub fn pop_front(&mut self) -> Option<T> {
        return self.items.pop_front();
    }

    pub fn pop_back(&mut self) -> Option<T> {
        return self.items.pop_back();
    }

    pub fn is_full(&self) -> bool {
        return self.items.len() == self.capacity;
    }

    pub fn clear(&mut self) -> Result<(), &'static str> {
        self.items.clear();
        Ok(())
    }
}
