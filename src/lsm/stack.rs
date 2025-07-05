pub struct Stack<T> {
    stack: Vec<T>,
    size: usize,
}

impl<T> Stack<T> {

    pub fn new(stack_size: usize) -> Stack<T> {
        Stack { stack: Vec::with_capacity(stack_size), size: stack_size }
    }
    pub fn push(&mut self, item: T) {
        if self.stack.len() >= self.size {
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