

pub fn run(cmnd: String, raw_input: String) {
    let cmnd_raw = cmnd.as_str();

    match cmnd_raw {
        "create" => create_command(raw_input),
        _ => println!("yoo")
    }
}

//commands (private)

fn create_command(_raw_input: String) {
    
    
}