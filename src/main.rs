use clap::Parser;
use colored::Colorize;

mod commands;
mod context;

fn main() {
    let cli = commands::CLI::parse();
    cli.run_command().unwrap_or_else(|x| {
        println!("{}", format!("{}", x).red());
        ()
    });
}
