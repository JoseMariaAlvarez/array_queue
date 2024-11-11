const ARRAY_SIZE: usize = 5;

#[derive(Debug)]
pub struct ArrayQueue<T: Copy + Default> {
    data: [T; ARRAY_SIZE],
    head: usize,
    elements: usize,
}

impl<T: Copy + Default> ArrayQueue<T> {
    pub fn new() -> Self {
         ArrayQueue {
            // data: std::array::from_fn(|_| T::default()),
            data: [T::default(); ARRAY_SIZE],
            head: 0,
            elements: 0,
        }
    }

    pub fn enqueue(&mut self, value: T) {
        // if self.elements == ARRAY_SIZE {
        //     panic!("Queue is full");
        // }
        // if self.elements == ARRAY_SIZE {
        //     return Error::new("Queue is full");
        // }
        if self.elements < ARRAY_SIZE {
            self.data[(self.head + self.elements) % ARRAY_SIZE] = value;
            self.elements += 1;
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.elements == 0 {
            return None;
        }
        let value = self.data[self.head].clone();
        self.head = (self.head + 1) % 5;
        self.elements -= 1;
        Some(value)
    }

    pub fn peek(&self) -> Option<&T> {
        if self.elements == 0 {
            return None;
        }
        Some(&self.data[self.head])
    }

    pub fn is_empty(&self) -> bool {
        self.elements == 0
    }
}

impl<T: Copy + Default> Drop for ArrayQueue<T> {
    fn drop(&mut self) {
        println!("Dropping ArrayQueue");
    }
}


