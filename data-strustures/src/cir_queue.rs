#[derive(Debug)]
pub struct Queue {
    size: isize,
    fp: isize,
    rp: isize,
    vector: Vec<isize>,
}

impl Queue {
    pub fn new(size: isize) -> Queue {
        if size < 1 {
            panic!("Queue size too small...!")
        }
        Queue {
            size: size,
            fp: -1,
            rp: -1,
            vector: vec![0; size as usize],
        }
    }

    pub fn enqueue(&mut self, data: isize) -> isize {
        if self.is_full() {
            panic!("Queue full")
        }
        if self.is_empty() {
            (self.fp, self.rp) = (0, 0);
            self.vector[self.rp as usize] = data;
            return data;
        }

        self.rp = (self.rp + 1) % self.size;
        self.vector[self.rp as usize] = data;
        return data;
    }

    pub fn dequeue(&mut self) -> isize {
        if self.is_empty() {
            panic!("Queue empty")
        }
        let data = self.vector[self.fp as usize];
        if self.fp == self.rp {
            (self.fp, self.rp) = (-1, -1);
        } else {
            self.fp = (self.fp + 1) % self.size;
        }
        return data;
    }

    pub fn is_empty(&self) -> bool {
        if self.fp == -1 && self.rp == -1 {
            return true;
        }
        return false;
    }

    pub fn is_full(&self) -> bool {
        if ((self.rp + 1) % self.size) == self.fp {
            return true;
        }
        return false;
    }
}
