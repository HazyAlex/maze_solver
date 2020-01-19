// STACK
#[derive(Default)]
pub struct Stack<T> {
    item: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { item: Vec::new() }
    }

    pub fn push(&mut self, x: T) {
        self.item.push(x)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.item.pop()
    }

    pub fn empty(&self) -> bool {
        self.item.is_empty()
    }
}

// QUEUE
#[derive(Default)]
pub struct Queue<T> {
    item: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self { item: Vec::new() }
    }

    pub fn enqueue(&mut self, x: T) {
        self.item.push(x);
    }

    pub fn dequeue(&mut self) -> T {
        if self.item.is_empty() {
            panic!("Can't dequeue an empty queue!");
        }

        self.item.remove(0)
    }

    pub fn empty(&self) -> bool {
        self.item.is_empty()
    }
}
