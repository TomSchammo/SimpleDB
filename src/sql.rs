#[derive(Debug)]
pub enum ParsingError {
    UnrecognizedStatement,
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
        ["insert", rest @ ..] => {
            println!("insert");
            println!("{:?}", rest);
            Ok(Statement {
                r#type: StatementType::Insert,
            })
        }
        _ => Err(ParsingError::UnrecognizedStatement),
    }
}
