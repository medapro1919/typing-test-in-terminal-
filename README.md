# Rust Type

A simple terminal-based typing test written in Rust using Ratatui.

## Features

- Randomly generated typing prompts
- Customisable test length
- Free typing mode
- Live words per minute (WPM) counter
- Live accuracy calculation
- Keyboard input with backspace support
- Custom orange terminal interface

## Requirements

- Rust and Cargo
- A terminal that supports colours
- A `words.txt` file in the project directory

## Installation

Clone the repository:

```bash
git clone https://github.com/medapro1919/typing-test-in-terminal-.git
cd typing-test-in-terminal-
```

Build and run:

```bash
cargo run
```

## Usage

When prompted, enter the number of words you want to type.

- Enter a number to generate a typing test.
- Enter `0` to use free typing mode.

### Controls

| Key | Action |
|---|---|
| Enter | Finish the test |
| Backspace | Delete a character |
| Esc | Exit the test |

## Word List

The program loads words from `words.txt`.

To customise the word list, edit `words.txt` or replace it with your own file. Each word should be on a separate line.

## Dependencies

- [rand](https://crates.io/crates/rand)
- [ratatui](https://crates.io/crates/ratatui)
- [crossterm](https://crates.io/crates/crossterm)

## Author

Jacob Wheeler
