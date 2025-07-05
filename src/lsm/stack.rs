const STACK_SIZE: usize = 128; // artificial limit

pub struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    pub fn push(&mut self, item: T) {
        if self.stack.len() >= STACK_SIZE {
            panic!("stack overflow");
        }
        self.stack.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}