use clap::{App, Arg, ArgMatches};
use std::io::{self, Write, BufRead};



pub struct Config {
    pub file: String,
    pub program: String,
    pub addon: String,
}

fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Make sure the prompt is displayed immediately
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_owned()
}

pub fn parse_args() -> Config {
    let matches = App::new("Office Template Helper")
        .version("1.0")
        .author("Your Name <you@example.com>")
        .about("Helps modify Office template files")
        .arg(Arg::with_name("file")
            .short('f')
            .long("file")
            .value_name("FILE")
            .help("Specifies the file to operate on")
            .takes_value(true))
        .arg(Arg::with_name("program")
            .short('p')
            .long("program")
            .value_name("PROGRAM")
            .help("Specifies the program (e.g., 'visio')")
            .takes_value(true))
        .arg(Arg::with_name("addon")
            .short('a')
            .long("addon")
            .value_name("ADDON")
            .help("Specifies the addon to apply (e.g., 'brainstorm')")
            .takes_value(true))
        .get_matches();

    let file = matches.value_of("file").map_or_else(
        || get_input("Enter the file path: "),
        ToString::to_string,
    );

    let program = matches.value_of("program").map_or_else(
        || get_input("Enter the program name: "),
        ToString::to_string,
    );

    let addon = matches.value_of("addon").map_or_else(
        || get_input("Enter the addon name: "),
        ToString::to_string,
    );

    Config { file, program, addon }
}

pub fn end_of_program() {
    // Wait specifically for the Enter key
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    iterator.next().unwrap().unwrap(); // Read one line, which includes the newline character
}
