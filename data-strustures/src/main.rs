pub mod cir_queue;
use crate::cir_queue::Queue;

fn main() {
    println!("hello");

    let mut q1 = Queue::new(5);

    println!("{:?}", q1);
    q1.enqueue(1);
    println!("{:?}", q1);
    q1.enqueue(2);
    println!("{:?}", q1);
    q1.enqueue(3);
    q1.enqueue(4);
    q1.enqueue(5);

    println!("{}", q1.dequeue());
    println!("{:?}", q1);
    println!("{}", q1.dequeue());
    println!("{:?}", q1);
    println!("{}", q1.dequeue());
    q1.enqueue(10);
    q1.enqueue(20);
    println!("{:?}", q1);
}
