use clap::{Arg, Command};
use std::io::BufRead;

fn main() {
    let args = config_args();

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

/// Define arguments configuration, returning the matches
fn config_args() -> clap::ArgMatches {
    Command::new("Boo")
        .about("Display a line of text")
        .author("Luiz Carlos Lima")
        .version("0.0.1")
        .arg(
            Arg::new("Text Line")
                .help("Text line to display")
                .exclusive(true)
                .required(false),
        )
        .get_matches()
}
