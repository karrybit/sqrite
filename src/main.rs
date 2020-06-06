use std::io::Write;

fn main() {
    loop {
        print_prompt();
        let mut command = String::new();
        std::io::stdin().read_line(&mut command).unwrap_or_else(|e| panic!("{}", e));
        command = command.trim().to_string();
        if command.get(0..1) == Some(".") {
            match do_meta_command(&command) {
                Ok(_) => (),
                Err(_) => {
                    println!("Unrecognized command '{}'", command);
                    continue;
                }
            }
        }
        let statement = match prepare_statement(&command) {
            Ok(statement_type) => statement_type,
            Err(_) => {
                println!("Unrecognized keyword at start of '{}'", command);
                continue;
            }
        };

        execute_statement(statement);
        println!("Executed.");
    }
}
fn print_prompt() {
    print!("db > ");
    std::io::stdout().flush().unwrap();
}
enum SqriteError {
    UnrecognizedCommand,
    UnrecognizedStatement,
}
fn do_meta_command(command: &str) -> Result<(), SqriteError> {
    if command == ".exit" {
        std::process::exit(0);
    }
    Err(SqriteError::UnrecognizedCommand)
}

enum StatementType {
    Select,
    Insert,
}

fn prepare_statement(command: &str) -> Result<StatementType, SqriteError> {
    match command {
        "select" => Ok(StatementType::Select),
        _ if command.get(0..6) == Some("insert") => Ok(StatementType::Insert),
        _ => Err(SqriteError::UnrecognizedStatement)
    }
}

fn execute_statement(statement: StatementType) {
    match statement {
        StatementType::Select => println!("This is where we would do an select"),
        StatementType::Insert => println!("This is where we would do an insert"),
    }
}
