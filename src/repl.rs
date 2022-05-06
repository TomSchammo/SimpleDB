use std::io::{stdin, stdout, Write};

const EXIT_SUCCESS: i32 = 0;

fn read_input() -> String {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Invalid input!");

    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }

    // windows uses \r\n for new lines whereas unix only uses \n
    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }

    input
}

fn print_prompt() {
    print!("db > ");
    let _ = stdout().flush();
}

fn parse_command(command: &str) {
    if !command.is_empty() {
        match (command.chars().next().expect("Invalid command!"), command) {
            ('.', ".exit") => {
                std::process::exit(EXIT_SUCCESS);
            }

            ('.', _) => {
                println!("Unrecognized meta-command: '{command}'.");
            }

            _ => {
                println!("Unrecognized command: '{command}'.");
            }
        }
    }
}

pub fn run() {
    loop {
        print_prompt();
        let input = read_input();
        parse_command(&input);
    }
}
