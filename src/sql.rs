const COLUMN_USERNAME_SIZE: usize = 32;
const COLUMN_EMAIL_SIZE: usize = 255;

#[derive(Debug)]
pub enum ParsingError {
    UnrecognizedStatement,
    InvalidId,
    InvalidUsername,
    InvalidEmail,
}

enum StatementType {
    Insert,
    Select,
}

pub struct Statement {
    r#type: StatementType,
}

pub fn prepare_statement(command: &str) -> Result<Statement, ParsingError> {
    match command.split(' ').collect::<Vec<&str>>().as_slice() {
        ["select", rest @ ..] => {
            println!("select");
            println!("{:?}", rest);
            Ok(Statement {
                r#type: StatementType::Select,
            })
        }
        ["insert", id, username, email] => {
            let id_result = id.parse::<u32>();

            if id_result.is_err() {
                println!("id must be an unsigned integer, found '{id}'");
                return Err(ParsingError::InvalidId);
            }

            if username.len() > COLUMN_USERNAME_SIZE {
                println!(
                    "username cannot be longer than {} characters '{}' has a length of {}",
                    COLUMN_USERNAME_SIZE,
                    username,
                    username.len()
                );
                return Err(ParsingError::InvalidUsername);
            }

            if email.len() > COLUMN_EMAIL_SIZE {
                println!(
                    "email cannot be longer than {} characters '{}' has a length of {}",
                    COLUMN_EMAIL_SIZE,
                    email,
                    email.len()
                );
                return Err(ParsingError::InvalidEmail);
            }

            Ok(Statement {
                r#type: StatementType::Insert,
            })
        }
        ["insert", rest @ ..] => {
            println!("invalid insert statement");
            println!("{:?}", rest);
            return Err(ParsingError::UnrecognizedStatement);
        }
        _ => Err(ParsingError::UnrecognizedStatement),
    }
}
