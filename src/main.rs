pub mod array_int_queue;
pub mod array_queue;
pub mod array_queue_box;
pub mod array_queue_box_option;
pub mod simple_list;
pub mod ok_stack_generic;

fn testing_references(){

    #[derive(Debug)]
    struct CajaDatos{
        d : i32,
        sig : Option<Box<CajaDatos>>
    }

    let mut cd = CajaDatos{d: 1, sig:None};
    let e = & mut cd;
    e.d = 3;
    e.sig = Some(Box::new(CajaDatos{d: 2, sig:None}));
    let d = & mut e.sig;
    *d = Some(Box::new(CajaDatos{d: 3, sig:None}));
    println!("d: {:?}", d);
    println!("cd: {:?}", cd);

}

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
    y: i32
}

impl Pair {
    fn new(x: i32, y: i32) -> Self {
        Pair { x, y}
    }
}

fn testing_array_queue_pair(){

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

#[derive(Debug, Default)] 
struct Triplet {
    x: i32,
    y: i32,
    z: String
}

impl Triplet {
    fn new(x: i32, y: i32, z: String) -> Self {
        Triplet { x, y, z}
    }
}

fn testing_array_queue_box_option(){

    use array_queue_box_option::ArrayQueueBoxOption;
    
        let mut a_q = ArrayQueueBoxOption::<Triplet>::new();
        println!("{:?}", a_q);
        for i in 0..5 {
            a_q.enqueue(Triplet::new(i, i, i.to_string()));
        }
        println!("{:?}", a_q);
        a_q.enqueue(Triplet::new(5, 5, 5.to_string()));
        println!("{:?}", a_q);
        println!("{:?}", a_q.peek());
        println!("{:?}", a_q.dequeue());
        println!("{:?}", a_q.peek());
        a_q.dequeue();
        a_q.dequeue();
        println!("{:?}", a_q);
        a_q.enqueue(Triplet::new(6, 6, 6.to_string()));
        println!("{:?}", a_q);
        a_q.enqueue(Triplet::new(7, 7, 7.to_string()));
        println!("{:?}", a_q);
        a_q.enqueue(Triplet::new(8, 8, 8.to_string()));
        println!("{:?}", a_q);
        a_q.enqueue(Triplet::new(9, 9, 9.to_string()));
        println!("{:?}", a_q);
        println!("{:?}", a_q.is_empty());
        while !a_q.is_empty() {
            println!("{:?}", a_q.dequeue());
        }
        println!("{:?}", a_q.dequeue());
}

fn testing_ok_stack_generics(){

    use ok_stack_generic::List;
    use ok_stack_generic::Node;

//    let mut a_q: List<Triplet> = List::new();
    let mut a_q: List<Triplet> = List{ head : None};
    a_q = List{ head : Some(Box::new(Node{ elem: Triplet::new(0, 0, "0".to_string()), next: None}))};
    println!("{:?}", a_q);
    for i in 1..3 {
        a_q.push(Triplet::new(i, i, i.to_string()));
    }
    a_q.push_last(Triplet::new(15, 15, 15.to_string()));
    a_q.traverse_queue();
    println!("{:?}", a_q);
    a_q.traverse_queue();
    let trip = a_q.pop();
    println!("{:?}", a_q);
    println!("{:?}", a_q.peek());
    println!("{:?}", a_q.pop());
    println!("{:?}", a_q.peek());
    let trip =a_q.pop();
    let trip =a_q.pop();
    println!("{:?}", a_q);
    a_q.push(Triplet::new(6, 6, 6.to_string()));
    println!("{:?}", a_q);
    a_q.push(Triplet::new(7, 7, 7.to_string()));
    println!("{:?}", a_q);
    a_q.push(Triplet::new(8, 8, 8.to_string()));
    println!("{:?}", a_q);
    a_q.push(Triplet::new(9, 9, 9.to_string()));
    println!("{:?}", a_q);
    println!("{:?}", a_q.is_empty());
    while !a_q.is_empty() {
        println!("{:?}", a_q.pop());
    }
    println!("{:?}", a_q.pop());
    
}

fn main() {

    println!("beginning of main");

    // testing_array_int_queue();
    // testing_array_queue_pair();
    // testing_ok_stack_generics();
    testing_array_queue_box_option();
    println!("end of main");


    
}

#[cfg(test)]      
mod test {
    use crate::ok_stack_generic::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
