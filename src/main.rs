use std::io::Write;

fn main() {
    loop {
        print_prompt();
        let mut command = String::new();
        std::io::stdin().read_line(&mut command).unwrap_or_else(|e| panic!("{}", e));
        command = command.trim().to_string();
        if command == ".exit" {
            break;
        } else {
            println!("Unrecognized command '{}'.", command);
        }
    }
}

fn print_prompt() {
    print!("db > ");
    std::io::stdout().flush().unwrap();
}
