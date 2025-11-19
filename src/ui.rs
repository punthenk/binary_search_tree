use std::io::{self, Write};

pub fn ask_task_input() -> (u32, String) {
    let mut description = String::new();
    print!("name: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut description)
        .expect("Not found");
    

    let priority_u32 = ask_task_priority();

    return (priority_u32, description);
}

pub fn ask_task_priority() -> u32 {
    let mut priority = String::new();
    print!("priority: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut priority)
        .expect("Not found");

    let priority_u32: u32 = loop {
        match priority.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Not a number. Try again");
                print!("priority: ");
                io::stdout().flush().unwrap();
                let mut new_input = String::new();
                io::stdin().read_line(&mut new_input).unwrap();
                priority = new_input;
            }
        }
    };

    return priority_u32;
}


pub fn get_command(prompt: &str) -> String {
    print!("{} ", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failded to read line");

    input.trim().to_string()
}
