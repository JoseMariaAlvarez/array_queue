use std::ops::Deref;

const ARRAY_SIZE: usize = 5;

#[derive(Debug, Default, Copy, Clone)]
enum Node<T>{
    #[default]
    Nil,
    Element(T)
}

#[derive(Debug)]
pub struct ArrayQueueLifetime<T: Default> {
    data: [Node<Box<T>>; ARRAY_SIZE],
    head: usize,
    elements: usize,
}

// impl<T: Default> ArrayQueueLifetime<T> {
//     pub fn new() -> Self {
//         ArrayQueueLifetime {
//             // data: std::array::from_fn(|_| Node::default()),
//             data: [const{Node::default()}; ARRAY_SIZE],
//             head: 0,
//             elements: 0,
//         }
//     }

//     pub fn enqueue(&mut self, value: T) {
//         // if ARRAY_SIZE <= self.elements {
//         //     panic!("Queue is full");
//         // }
//         // if ARRAY_SIZE <= self.elements {
//         //     return Error::new("Queue is full");
//         // }
//         if self.elements < ARRAY_SIZE {
//             self.data[(self.head + self.elements) % ARRAY_SIZE] = Node::Element(Box::new(value));
//             self.elements += 1;
//         }
//     }

//     pub fn dequeue(&mut self) -> Node<&T> {
//         if self.elements == 0 {
//             return Node::Nil;
//         }
//         let value = &self.data[self.head];
//         self.head = (self.head + 1) % 5;
//         self.elements -= 1;
//         if let Node::Element(v) = value {
//             return Node::Element(v.deref());
//         } else {
//             return Node::Nil;
//         }
//     }

//     pub fn peek(&self) -> Node<&T> {
//         if self.elements == 0 {
//             return Node::Nil;
//         }
//         match &self.data[self.head] {
//             Node::Nil => Node::Nil,
//             Node::Element(v) => Node::Element(v.deref())
//         }
//     }

//     pub fn is_empty(&self) -> bool {
//         self.elements == 0
//     }
// }

// impl<T: Default> Drop for ArrayQueueLifetime<T> {
//     fn drop(&mut self) {
//         println!("Dropping ArrayQueue");
//     }
// }

