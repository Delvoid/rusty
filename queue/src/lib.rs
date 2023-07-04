use std::collections::LinkedList;

pub struct Queue<T> {
    element: LinkedList<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            element: LinkedList::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.element.push_back(value);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.element.pop_front()
    }

    pub fn peek(&self) -> Option<&T> {
        self.element.front()
    }

    pub fn length(&self) -> usize {
        self.element.len()
    }

    pub fn is_empty(&self) -> bool {
        self.element.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut queue = Queue::new();
        assert_eq!(queue.is_empty(), true);
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        assert_eq!(queue.is_empty(), false);
        assert_eq!(queue.peek(), Some(&1));
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None);
        assert_eq!(queue.is_empty(), true);
    }
}
```
