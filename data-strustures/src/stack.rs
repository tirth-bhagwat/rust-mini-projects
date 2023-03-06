#[derive(Debug)]
pub struct Stack {
    tos: isize,
    size: isize,
    vector: Vec<isize>,
}

impl Stack {
    pub fn new(size: isize) -> Stack {
        Stack {
            tos: -1,
            size: size,
            vector: Vec::new(),
        }
    }

    pub fn push(&mut self, data: isize) -> isize {
        if self.tos < self.size - 1 {
            self.tos += 1;
            self.vector.push(data);
            return data;
        } else {
            panic!("Stack Overflow")
        }
    }

    pub fn pop(&mut self) -> isize {
        if self.tos != -1 {
            let x = self.vector[self.tos as usize];
            self.tos -= 1;
            return x;
        } else {
            panic!("Stack Underflow")
        }
    }

    pub fn print(&self) {
        if self.tos < 0 {
            println!("");
            return;
        }
        for i in 0..(self.tos + 1) {
            print!("{} ", self.vector[i as usize]);
        }
        println!("");
    }
}
