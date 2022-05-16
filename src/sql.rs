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
    /// Type of the statement (currently only `select` and `insert` are suppported)
    r#type: StatementType,
    /// A Row that should be inserted into the taple. Is `None` when the type of the statement is
    /// `select` and `Some` when it is `insert`.
    row_to_insert: Option<Row>,
}

pub struct Row {
    id: u32,
    username: [u8; COLUMN_USERNAME_SIZE],
    email: [u8; COLUMN_EMAIL_SIZE],
}

pub fn prepare_statement(command: &str) -> Result<Statement, ParsingError> {
    match command.split(' ').collect::<Vec<&str>>().as_slice() {
        ["select", rest @ ..] => {
            println!("select");
            println!("{:?}", rest);
            Ok(Statement {
                r#type: StatementType::Select,
                row_to_insert: None,
            })
        }
        ["insert", id, username, email] => {
            let id_result = id.parse::<u32>();

            if id_result.is_err() {
                println!("id must be an unsigned integer, found '{id}'");
                return Err(ParsingError::InvalidId);
            }

            let username_bytes = username.as_bytes();
            let email_bytes = email.as_bytes();

            if username_bytes.len() > COLUMN_USERNAME_SIZE {
                println!(
                    "username cannot be longer than {} bytes '{}' has a length of {}",
                    COLUMN_USERNAME_SIZE,
                    username,
                    username.len()
                );
                return Err(ParsingError::InvalidUsername);
            }

            if email_bytes.len() > COLUMN_EMAIL_SIZE {
                println!(
                    "email cannot be longer than {} bytes '{}' has a length of {}",
                    COLUMN_EMAIL_SIZE,
                    email,
                    email.len()
                );
                return Err(ParsingError::InvalidEmail);
            }

            Ok(Statement {
                r#type: StatementType::Insert,
                row_to_insert: Some(Row {
                    id: id_result.unwrap(),
                    username: store_username(username_bytes),
                    email: store_email(email_bytes),
                }),
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

/// Copies the username data from a slice into a fixed size array.
fn store_username(username: &[u8]) -> [u8; COLUMN_USERNAME_SIZE] {
    let mut username_bytes = [0u8; COLUMN_USERNAME_SIZE];

    { 0..username.len() }.for_each(|i| {
        username_bytes[i] = username[i];
    });

    username_bytes
}

/// Copies the email data from a slice into a fixed size array.
fn store_email(email: &[u8]) -> [u8; COLUMN_EMAIL_SIZE] {
    let mut email_bytes = [0u8; COLUMN_EMAIL_SIZE];

    { 0..email.len() }.for_each(|i| {
        email_bytes[i] = email[i];
    });

    email_bytes
}
