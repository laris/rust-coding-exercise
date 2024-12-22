use std::path::PathBuf;
use clap::{arg, command, value_parser, ArgAction, Command};

fn main() {
    //println!("Hello, world!");
    let matches = command!() // requires cargo feature
        .arg(arg!([name] "Optional name to operate on"))
        .arg(
            arg!(
            -c --config <FILE> "Sets a custom config file"
            )
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(
            -d --debug ... "Turn debugging information on"
        ))
        .subcommand(
            Command::new("test")
                .about("does testing things")
                .arg(arg!(-l --list "list test values").action(ArgAction::SetTrue)),
        )
        .get_matches();



}
