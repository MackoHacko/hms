use clap::{value_parser, Arg, ArgAction, Command};

pub const ADD_CMD_ID: &str = "add";
pub const ADD_CMD_ALIAS_ARG_ID: &str = "alias";
pub const ADD_CMD_SNIP_ARG_ID: &str = "snip";

pub fn add_cmd() -> Command {
    Command::new(ADD_CMD_ID)
        .about("Add a snip")
        .arg(
            Arg::new(ADD_CMD_ALIAS_ARG_ID)
                .value_parser(value_parser!(String))
                .short('a')
                .long("alias")
                .required(true)
                .help("Alias for the snip")
                .action(ArgAction::Set)
                .num_args(1),
        )
        .arg(
            Arg::new(ADD_CMD_SNIP_ARG_ID)
                .value_parser(value_parser!(String))
                .short('s')
                .long("snip")
                .required(true)
                .help("The snip to add")
                .action(ArgAction::Set)
                .num_args(1),
        )
}
