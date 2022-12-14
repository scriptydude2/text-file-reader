use std::{io};
mod  reader;
fn main() {
    //collecting input
    loop {
        let mut input = String::new();
        io::stdin()
         .read_line(&mut input)
         .expect("failed to read line");

        if input.contains("exit") {
            break;
        }else if input.contains("read") {
            reader::read_file("hello.txt")
        }
    }
}

