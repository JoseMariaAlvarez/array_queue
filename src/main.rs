pub mod array_int_queue;
pub mod array_queue;

fn testing_array_int_queue(){

    let mut a_q = array_int_queue::ArrayIntQueue::new();
    println!("{:?}", a_q);
    for i in 0..5 {
        a_q.enqueue(i);
    }
    println!("{:?}", a_q);
    a_q.enqueue(5);
    println!("{:?}", a_q);
    println!("{:?}", a_q.peek());
    println!("{:?}", a_q.dequeue());
    println!("{:?}", a_q.peek());
    a_q.dequeue();
    a_q.dequeue();
    println!("{:?}", a_q);
    a_q.enqueue(6);
    println!("{:?}", a_q);
    a_q.enqueue(7);
    println!("{:?}", a_q);
    a_q.enqueue(8);
    println!("{:?}", a_q);
    a_q.enqueue(9);
    println!("{:?}", a_q);
    println!("{:?}", a_q.is_empty());
    while !a_q.is_empty() {
        println!("{:?}", a_q.dequeue());
    }
    println!("{:?}", a_q.dequeue());
    
}

#[derive(Debug, Default, Copy, Clone)] 
struct Pair {
    x: i32,
    y: i32,
}

impl Pair {
    fn new(x: i32, y: i32) -> Self {
        Pair { x, y }
    }
}

fn testing_array_queue_point(){

    let mut a_q = array_queue::ArrayQueue::<Pair>::new();
    println!("{:?}", a_q);
    for i in 0..5 {
        a_q.enqueue(Pair::new(i, i));
    }
    println!("{:?}", a_q);
    a_q.enqueue(Pair::new(5, 5));
    println!("{:?}", a_q);
    println!("{:?}", a_q.peek());
    println!("{:?}", a_q.dequeue());
    println!("{:?}", a_q.peek());
    a_q.dequeue();
    a_q.dequeue();
    println!("{:?}", a_q);
    a_q.enqueue(Pair::new(6, 6));
    println!("{:?}", a_q);
    a_q.enqueue(Pair::new(7, 7));
    println!("{:?}", a_q);
    a_q.enqueue(Pair::new(8, 8));
    println!("{:?}", a_q);
    a_q.enqueue(Pair::new(9, 9));
    println!("{:?}", a_q);
    println!("{:?}", a_q.is_empty());
    while !a_q.is_empty() {
        println!("{:?}", a_q.dequeue());
    }
    println!("{:?}", a_q.dequeue());
    
}

fn prueba(numero : &mut i32) {
    *numero = *numero + 1;
}

fn main() {

    let mut n = 5;
    prueba(& mut n);
    println!("{:?}", n);

    println!("beginning of main");
    testing_array_int_queue();
    testing_array_queue_point();
    println!("end of main");

    // let mut a_q : ArrayQueue<i32> = ArrayQueue::new();
    // println!("{:?}", a_q);
    // for i in 0..5 {
    //     a_q.enqueue(i);
    // }
    // println!("{:?}", a_q);
    // println!("{:?}", a_q.peek());
    // println!("{:?}", a_q.dequeue());
    // println!("{:?}", a_q.peek());
    // println!("{:?}", a_q.is_empty());
    // while !a_q.is_empty() {
    //     println!("{:?}", a_q.dequeue());
    // }
    // println!("{:?}", a_q.dequeue());

    // let mut a_q : ArrayQueue<String> = ArrayQueue::new();

    // println!("{:?}", a_q);
    // for i in 0..5 {
    //     a_q.enqueue("string" + i.to_string());
    // }

}



// pub struct ArrayQueue<T> {
//     data: Vec<T>,
//     head: usize,
//     tail: usize,
// }

// impl<T> ArrayQueue<T> {
//     pub fn new() -> Self {
//         ArrayQueue {
//             data: Vec::new(),
//             head: 0,
//             tail: 0,
//         }
//     }

//     pub fn enqueue(&mut self, value: T) {
//         self.data.push(value);
//         self.tail += 1;
//     }

//     pub fn dequeue(&mut self) -> Option<T> {
//         if self.head == self.tail {
//             return None;
//         }
//         let value = self.data[self.head];
//         self.head += 1;
//         Some(value)
//     }

//     pub fn peek(&self) -> Option<&T> {
//         if self.head == self.tail {
//             return None;
//         }
//         Some(&self.data[self.head])
//     }

//     pub fn is_empty(&self) -> bool {
//         self.head == self.tail
//     }
// }

// fn main() {
//     println!("Hello, world!");
// }
