use super::display_mode::DisplayMode;
use clap::Parser;

#[derive(Parser, Debug)]
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
    #[arg(short, long, default_value_t=DisplayMode::Small)]
    pub display_mode: DisplayMode,
}
