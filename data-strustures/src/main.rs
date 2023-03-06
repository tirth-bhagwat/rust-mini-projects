pub mod stack;
use crate::stack::Stack;

fn main() {
    println!("hello");

    let mut s1 = Stack::new(5);

    s1.push(10);
    s1.push(11);
    s1.push(12);
    s1.push(13);
    s1.push(14);

    println!("Started popping ...");
    let x = s1.pop();
    println!("{x} ");


    s1.print()
}
