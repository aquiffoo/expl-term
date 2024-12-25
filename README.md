# expl-term

A blazingly fast™ file explorer on the CLI.

## Features

- Navigate directories and list files
- Search for files within the current directory
- Open files with Vim
- Simple and intuitive keyboard controls

## Installation

To install `expl-term`, clone the repository and build it using Cargo:

```sh
git clone https://github.com/yourusername/expl-term.git
cd expl-term
cargo build --release
```

## Usage
Run the application:
```sh
cargo run --release
```

## User Interface
- Arrow Up/Down: File navigation
- Arrow Left/Right: go back or forth in a directory
- Enter: File selection
- Pressing `q` quits the app
- Pressing `s` opens search

## Dependencies
This project uses crossterm for UI rendering.
