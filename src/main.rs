use clap::{Arg, Command};
use std::{fmt, io::BufRead, process};

fn main() {
    let args = config_args();
    let line = args.get_one::<String>("Text Line");

    display_line(line);
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

/// Display the text line given as argument or from stdin
fn display_line<T>(line: Option<T>)
where
    T: fmt::Display,
{
    if let Some(line_) = line {
        println!("{}", line_);
        process::exit(0);
    }

    let stdin = std::io::stdin();
    let reader = stdin.lock();

    for line_ in reader.lines() {
        if let Ok(line) = line_ {
            println!("{}", line);
        }
        process::exit(0);
    }
}
