![Banner](./assets/banner.jpeg)

# Hold my Snip!

Welcome to **Hold my Snip!**, a Rust-based CLI tool designed to enhance your development workflow by efficiently managing code snippets. With **Hold my Snip!**, you can easily store, retrieve, and organize snippets of code (or any text really 🤷‍♂️) under convenient aliases.

## Features

- **🏗️ Manage:** Easily store and organize your code snippets under aliases with a user friendly CLI
- **🔮 Find:** Search and view your snippets in a terminal GUI
- **🪣 Ingest:** Import snippets from csv
- **📊 Dashboards:** Tracks your most used snippets and makes them viewable in a chart

## Installation
```bash
cargo install hms
```

## Usage

```bash
Usage:hms [OPTIONS] [COMMAND]

Commands:
  add     Adds a new snip with an alias, can be piped eg: `echo snip | add -a alias`
  import  Import snips
  stats   Snip stats
  help    Print this message or the help of the given subcommand(s)

Options:
  -d, --display-mode <DISPLAY_MODE>
          [default: small]

          Possible values:
          - large: Full screen mode
          - small: Small mode; draws gui starting from current cursor line

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

### Commands

### add
Adds a new snip with an alias, can be piped eg: `echo snip | add -a alias`

***Note:*** Aliases must be unique and no more than 50 characters in length.
```bash
Usage: hms add --alias <ALIAS> [SNIP]

Arguments:
  [SNIP]  The snip to add

Options:
  -a, --alias <ALIAS>  Alias for the snip being added
  -h, --help           Print help
```

### import
Import snips

***Note:*** Aliases must be unique and no more than 50 characters in length.
```bash
Usage: hms import <COMMAND>

Commands:
  csv   Import snips from csv file
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### stats
Snip stats
```bash
Usage: hms stats <COMMAND>

Commands:
  top-ten  Display barchart for top ten most accessed snips, only considers snips accessed at least once
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## Contributing
This project requires the use of Rust's nightly toolchain for code formatting, which can be installed and managed via `rustup`.

To install the nightly toolchain, open your terminal and run the following command:

```bash
rustup install nightly
````

Then use this to run the formatter:

```bash
cargo +nightly fmt
````

## License
**Hold my Snip!** is open source and available under the MIT License. Feel free to use, modify, and distribute it as you see fit.
