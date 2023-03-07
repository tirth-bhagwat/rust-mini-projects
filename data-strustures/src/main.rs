mod stack_generic;
use crate::stack_generic::Stack;

fn main() {
    println!("hello");

    let mut s1: Stack<isize> = Stack::new(5);
    s1.push(10);
    s1.push(10);
    s1.push(10);
    s1.push(10);
    s1.push(10);

    println!("{}", s1.pop());
    println!("{}", s1.pop());
    println!("{}", s1.pop());
    println!("{}", s1.pop());

    s1.push(13);
    s1.push(11);
    println!("{:?}", s1);

    let mut s2: Stack<String> = Stack::new(5);
    s2.push(String::from("a"));
    s2.push(String::from("b"));
    s2.push(String::from("c"));
    s2.push(String::from("d"));
    println!("{}", s2.pop());
    println!("{:?}", s2);
}
