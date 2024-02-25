use super::display_mode::DisplayMode;
use clap::{
    builder::{
        styling::{AnsiColor, Effects},
        Styles,
    },
    command, Args as ClapArgs, Parser, Subcommand,
};
use std::path::PathBuf;

fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Green.on_default() | Effects::UNDERLINE)
        .usage(AnsiColor::Green.on_default())
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Magenta.on_default())
}

#[derive(Debug, Parser)]
#[command(
    name = "Hold my Snip!",
    version = env!("HMS_GIT_INFO"),
    about = clap::crate_description!(),
    author = clap::crate_authors!(),
    styles = styles(),
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
    /// Import snips
    Import(ImportArgs),
    /// Snip stats
    Stats(StatsArgs),
}

#[derive(Debug, ClapArgs)]
pub struct ImportArgs {
    #[command(subcommand)]
    pub command: ImportCommand,
}

#[derive(Debug, Subcommand)]
pub enum ImportCommand {
    /// Import snips from csv file
    Csv {
        #[arg(short, long, help = "Path to csv file")]
        file: PathBuf,
    },
}

#[derive(Debug, ClapArgs)]
pub struct StatsArgs {
    #[command(subcommand)]
    pub command: StatsCommand,
}

#[derive(Debug, Subcommand)]
pub enum StatsCommand {
    /// Display barchart for top ten most accessed snips, only considers snips accessed at least once
    TopTen,
}
