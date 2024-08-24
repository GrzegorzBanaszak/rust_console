use std::io::{self, Write};

struct Command(String, fn());
#[allow(dead_code)]
fn display_commands() {
    let commands = get_commands();
    for command in commands {
        println!("{}", command.0);
    }
}

#[allow(dead_code)]
fn get_commands() -> Vec<Command> {
    let mut commands = Vec::new();
    commands.push(Command("help".to_string(), display_commands));
    commands
}

fn main() {
    let command = wait_for_commend();

    check_command(command);
}

fn wait_for_commend() -> String {
    print!("$ ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input
}

//Wypisz liste komend
fn check_command(command_name: String) {
    let commands = get_commands();

    for command in commands {
        if command.0.trim() == command_name.trim() {
            command.1();
        }
    }
}

//Stworzyć aplikacje z kilkoma komendami

//Wypisz pliki z lokacji

//Zapisz do pliku txt

//Podejrzyj zawartość pliku
