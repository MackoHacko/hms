![Banner](./assets/banner.jpeg)

# Hold my Snip!

Welcome to **Hold my Snip!**, a Rust-based tool designed to enhance your development workflow by efficiently managing code snippets. With **Hold my Snip!**, you can easily store, retrieve, and organize snippets of code under convenient aliases, streamlining your coding process.

## Features

- **Snippet Management:** Easily store and organize your code snippets with customizable aliases.
- **SQLite Database:** Utilizes an SQLite database for reliable and fast storage.
- **Automatic Directory Creation:** Automatically creates a dedicated directory in your home folder for the database and configuration files, ensuring your snippets are always backed up and easily accessible.

## Usage
```bash
Usage:hms [OPTIONS] [COMMAND]

Commands:
  add   Adds a new snip with an alias, can be piped eg: `echo snip | add -a alias`
  help  Print this message or the help of the given subcommand(s)

Options:
  -d, --display-mode <DISPLAY_MODE>  [default: small] [possible values: large, small]
  -h, --help                         Print help (see more with '--help')
  -V, --version                      Print version
```

## Contributing
This project requires the use of Rust's nightly toolchain for code formatting, which can be installed and managed via `rustup`.

To install the nightly toolchain, open your terminal and run the following command:

```bash
rustup install nightly
````

## License
**Hold my Snip!** is open source and available under the MIT License. Feel free to use, modify, and distribute it as you see fit.
