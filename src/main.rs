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

fn match_command(command: &str) {
    match command {
        ".exit" => {
            std::process::exit(EXIT_SUCCESS);
        }
        _ => {
            println!("Unrecognized command: '{command}'.");
        }
    }
}

fn main() {
    loop {
        print_prompt();
        let input = read_input();
        match_command(&input);
    }
}