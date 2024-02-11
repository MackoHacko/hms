use super::display_mode::DisplayMode;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "Hold my Snip!",
    version = env!("HMS_GIT_INFO"),
    about = clap::crate_description!(),
    author = clap::crate_authors!(),
    help_template("\
{before-help}{name}

Version: {version}
Author: {author-with-newline}
{about-with-newline}
{usage-heading}{usage}

{all-args}{after-help}
    ")
)]
pub struct Args {
    #[arg(short, long, default_value_t = DisplayMode::Small)]
    pub display_mode: DisplayMode,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Adds a new snip with an alias, can be piped eg: `echo snip | add -a alias`
    Add {
        #[arg(help = "The snip to add")]
        snip: Option<String>,

        #[arg(short, long, help = "Alias for the snip being added")]
        alias: String,
    },
}
