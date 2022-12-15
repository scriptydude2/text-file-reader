use std::{io};
mod commands;

fn main() {
    let commands: [String; 2] = ["exit".to_string(),"read".to_string()];
    //collecting input
    loop {
        let mut input = String::new();
        io::stdin()
         .read_line(&mut input)
         .expect("failed to read line");

         let first_word = input
      .split_whitespace()
      .next()
      .unwrap_or("");

      if commands.contains(&first_word.to_string()) {
          
      }
        
    }
}

