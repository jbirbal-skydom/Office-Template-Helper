use clap::{Arg, ArgMatches, Command};
use std::io::{self, BufRead, Write};
use std::path::Path;

pub struct Config {
    pub file: String,
    pub program: String,
    pub addon: String,
}

fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Make sure the prompt is displayed immediately
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_owned()
}

fn is_valid_path(s: &str) -> Result<(), String> {
    let path = Path::new(s);
    if path.exists() && path.is_file() {
        Ok(())
    } else {
        Err(format!(
            "The file '{}' does not exist or is not a valid file.",
            s
        ))
    }
}

fn get_valid_file_path() -> String {
    loop {
        let mut input = String::new();
        println!("Enter the file path: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if let Ok(_) = is_valid_path(input) {
            return input.to_string();
        } else {
            println!("The file '{}' does not exist or is not a valid file. Please try again.", input);
        }
    }
}

pub fn start() -> Config {
    let matches = parse_args();
    let arg_file = matches.get_one::<String>("file")
    .map(|s| s.to_string())
    .unwrap_or_else(|| get_valid_file_path());

    let arg_program = matches
        .get_one::<String>("program")
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| get_input("Enter the program name: "));

    let arg_addon = matches
        .get_one::<String>("addon")
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| get_input("Enter the addon name: "));

    Config {
        file: arg_file,
        program: arg_program,
        addon: arg_addon,
    }
}

fn parse_args() -> ArgMatches {
    Command::new("Office Template Helper")
        .version("1.0")
        .author("Jason Birbal <officehelper@skydom.ai>")
        .about("Helps modify Office template files")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("Specifies the file to operate on")
                .action(clap::ArgAction::Set)
                .value_parser(clap::builder::ValueParser::new(is_valid_path))
                .required(false),
        )
        .arg(
            Arg::new("program")
                .short('p')
                .long("program")
                .value_name("PROGRAM")
                .help("Specifies the program (e.g., 'visio')")
                .action(clap::ArgAction::Set)
                .required(false),
        )
        .arg(
            Arg::new("addon")
                .short('a')
                .long("addon")
                .value_name("ADDON")
                .help("Specifies the addon to apply (e.g., 'brainstorm')")
                .action(clap::ArgAction::Set)
                .required(false),
        )
        .get_matches()
}

pub fn end_of_program() {
    // Wait specifically for the Enter key
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    iterator.next().unwrap().unwrap(); // Read one line, which includes the newline character
}
