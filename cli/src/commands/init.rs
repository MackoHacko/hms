use clap::Command;

pub const INIT_CMD_ID: &str = "init";

pub fn init_cmd() -> Command {
    Command::new(INIT_CMD_ID).about("Initilize hms (required before first time use)")
}
