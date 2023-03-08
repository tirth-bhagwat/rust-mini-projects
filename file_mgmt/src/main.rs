use std::fs::*;
use std::io::prelude::*;
fn main() {
    // let p1 = "./files/a.txt";
    let p2 = "./files/a2.txt";
    let mut op_str = String::new();

    {
        let mut f1 = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(p2)
            .unwrap();
        f1.write("First line".as_bytes())
            .expect("Data cannot be written...!");
    }

    {
        let mut f1 = OpenOptions::new().append(true).open(p2).unwrap();
        f1.write("\nNewly added line".as_bytes())
            .expect("Cannot append to file");
    }

    {
        let mut f1 = OpenOptions::new().read(true).open(p2).unwrap();
        f1.read_to_string(&mut op_str).expect("cannot read file");
        println!("{op_str}");
    }
}
