use clap::{crate_description, crate_name, Command};

pub fn main_cmd() -> Command {
    Command::new(crate_name!())
        .about(crate_description!())
        .subcommand_required(true)
        .arg_required_else_help(true)
}
