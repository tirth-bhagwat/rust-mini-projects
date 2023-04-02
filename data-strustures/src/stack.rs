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

    pub fn pop(&mut self) -> Result<isize, Box<dyn std::error::Error>> {
        if self.tos != -1 {
            let x = self.vector[self.tos as usize];
            self.tos -= 1;
            return Ok(x);
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let stack = Stack::new(10);
        assert_eq!(stack.tos, -1, "Invalid tos");
        assert_eq!(stack.size, 10, "Invalid size of stack");
        assert_eq!(stack.vector.len(), 0, "Vector is not empty");
    }

    #[test]
    #[should_panic(expected = "Stack Overflow")]
    fn test_push() {
        let mut stack = Stack::new(5);

        assert_eq!(stack.push(10), 10, "Invalid push");
        assert_eq!(stack.push(20), 20, "Invalid push");
        assert_eq!(stack.push(30), 30, "Invalid push");
        assert_eq!(stack.push(40), 40, "Invalid push");
        assert_eq!(stack.push(50), 50, "Invalid push");

        assert_eq!(stack.push(50), 50, "Invalid push");
    }

    #[test]
    fn test_pop() -> Result<(), Box<dyn std::error::Error>> {
        let mut stack = Stack::new(5);

        assert_eq!(stack.push(10), 10, "Cannot push");
        assert_eq!(stack.push(20), 20, "Cannot push");
        assert_eq!(stack.push(30), 30, "Cannot push");
        assert_eq!(stack.push(40), 40, "Cannot push");
        assert_eq!(stack.push(50), 50, "Cannot push");

        assert_eq!(stack.pop()?, 50, "Invalid pop");
        assert_eq!(stack.pop()?, 40, "Invalid pop");
        assert_eq!(stack.pop()?, 30, "Invalid pop");
        assert_eq!(stack.pop()?, 20, "Invalid pop");
        assert_eq!(stack.pop()?, 10, "Invalid pop");

        assert!(stack.pop().is_err(), "Stack is not empty");

        Ok(())
    }
}
