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
                let result = super::sql::prepare_statement(command);

                match result {
                    Err(super::sql::ParsingError::UnrecognizedStatement) => {
                        println!("Unrecognized statement: '{command}'.");
                    }
                    Err(super::sql::ParsingError::InvalidId) => {
                        println!("Syntax error in '{command}', expected ID as first argument");
                    }
                    Err(super::sql::ParsingError::InvalidUsername) => {
                        println!(
                            "Syntax error in '{command}', expected username as second argument"
                        );
                    }
                    Err(super::sql::ParsingError::InvalidEmail) => {
                        println!("Syntax error in '{command}', expected email as third argument");
                    }
                    Ok(_statement) => {
                        println!("ok");
                    }
                }
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
