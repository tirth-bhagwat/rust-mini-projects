#[derive(Debug)]
pub struct Stack<T> {
    pub tos: isize,
    pub size: isize,
    pub vector: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new(size: isize) -> Stack<T> {
        if size < 0 {
            panic!("Stack size too samll...")
        }
        let s1: Stack<T> = Stack {
            tos: -1,
            size: size,
            vector: Vec::new(),
        };
        return s1;
    }
    pub fn push(&mut self, data: T) -> &T {
        if self.tos + 1 < self.size {
            self.tos += 1;
            self.vector.push(data);
            return &self.vector[self.tos as usize];
        } else {
            panic!("Stack Overflow")
        }
    }
    pub fn pop(&mut self) -> T {
        if self.tos - 1 == -1 {
            panic!("Stack Empty")
        } else {
            let tmp = match self.vector.pop() {
                Some(value) => value,
                _ => panic!("Error"),
            };
            self.tos -= 1;
            return tmp;
        }
    }
}
