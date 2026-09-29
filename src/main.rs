use clap::{Arg, Command};
use std::io::BufRead;

fn main() {
    let args = Command::new("Boo")
        .about("Display a line of text")
        .author("Luiz Carlos Lima")
        .version("0.1")
        .arg(
            Arg::new("Line Text")
                .help("A text line to display")
                .exclusive(true)
                .required(false),
        )
        .get_matches();

    let line_ = args.get_one::<String>("Line Text");

    if let Some(line) = line_ {
        println!("{}", line);
        std::process::exit(0);
    } else {
        let stdin = std::io::stdin();
        let reader = stdin.lock();

        for line_ in reader.lines() {
            if let Ok(line) = line_ {
                println!("{}", line);
            }
            std::process::exit(0);
        }
    }
}
