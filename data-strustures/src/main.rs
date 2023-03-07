mod cir_queue_option;
mod stack_generic;
use crate::cir_queue_option::Queue;

fn main() {
    let mut q1 = Queue::new(5);

    for i in 0..7 {
        let res = q1.enqueue(i as isize);
        match res {
            Some(value) => println!("Inserted {value}"),
            None => println!("Cannot insert {i}"),
        }
    }

    q1.enqueue(30 as isize).expect("Error Message");
    // above line will panic with given message if value is None

    q1.enqueue(40 as isize).unwrap();
    // above line will panic with a bulilt-in message
}
