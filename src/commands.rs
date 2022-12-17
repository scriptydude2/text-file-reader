use std::io;



pub fn run(cmnd: String) {
    let cmnd_raw = cmnd.as_str();

    match cmnd_raw {
        "exit" => exit_command(),
        _ => println!("yoo")
    }
}

//commands (private)

fn exit_command() {
    println!("bye bye im go sleepie");
    
}