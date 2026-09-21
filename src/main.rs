use clap::{Arg, ColorChoice, Command, builder::styling};
use colored::Colorize;
use std::io::BufRead;

fn main() {
    const STYLES: styling::Styles = styling::Styles::styled()
        .header(styling::AnsiColor::BrightBlue.on_default().bold())
        .usage(styling::AnsiColor::BrightGreen.on_default().bold())
        .literal(styling::AnsiColor::Yellow.on_default());

    let args = Command::new("Boo")
        .about("Display a line of text")
        .author("Luiz Carlos Lima")
        .version("0.1")
        .color(ColorChoice::Auto)
        .styles(STYLES)
        .arg(
            Arg::new("Line Text")
                .help("A text line to display")
                .exclusive(true)
                .required(false),
        )
        .get_matches();

    let line_ = args.get_one::<String>("Line Text");

    if let Some(line) = line_ {
        println!("{}", line.bright_green().bold());
        std::process::exit(0);
    } else {
        let stdin = std::io::stdin();
        let reader = stdin.lock();

        for line_ in reader.lines() {
            if let Ok(line) = line_ {
                println!("{}", line.bright_green().bold());
            }
            std::process::exit(0);
        }
    }
}
