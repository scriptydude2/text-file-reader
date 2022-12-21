use std::{fs::{File, self}};


pub fn run(cmnd: String, raw_input: String) {
    let cmnd_raw = cmnd.as_str();

    match cmnd_raw {
        "create" => create_command(raw_input),
        "read" => read_command(raw_input),
        _ => println!("error")
    }
}

//commands (private)

fn create_command(_raw_input: String) {
    let vec: Vec<&str> = _raw_input.split_whitespace().collect();
    
    println!("///Operation: {cmd}, with the argument {arg}. Creating {arg}.txt",cmd = vec[0],arg = vec[1]);

    let _file = File::create(format!("{name}.txt",name = vec[1]));
    
}
fn read_command(_raw_input: String) {
    let vec: Vec<&str> = _raw_input.split_whitespace().collect();
    println!("///Operation: {cmd}, with the argument {arg}. Reading {arg}.txt",cmd = vec[0],arg = vec[1]);

    let contents = fs::read_to_string(format!("{}.txt", vec[1]))
        .expect("Should have been able to read the file");
    println!("||Reading {file}||\n{content}", file = vec[1], content = contents);
}