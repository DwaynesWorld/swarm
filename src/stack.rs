pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    pub fn with_capacity(init_capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(init_capacity),
        }
    }
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }
}
