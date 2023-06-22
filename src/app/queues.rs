use super::doolist::DooItem;
use std::collections::VecDeque;

pub struct UndoQueue<T> {
    queue: VecDeque<T>,
    capacity: usize,
}

impl<T> UndoQueue<T> {
    pub fn new(capacity: usize) -> UndoQueue<T> {
        UndoQueue {
            queue: VecDeque::new(),
            capacity,
        }
    }

    pub fn push(&mut self, item: T) -> Result<(), &'static str> {
        if self.queue.len() < self.capacity {
            self.queue.push_front(item);

            return Ok(());
        }

        return Err("Queue has reached max capacity, cannot push items to queue");
    }

    pub fn pop(&mut self) -> Option<T> {
        return self.queue.pop_front();
    }
}
