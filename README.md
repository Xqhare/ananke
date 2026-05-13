# Ananke

Ananke is a fully featured to-do app written in Rust for all UNIX platforms.

<p align="center">
<img src="https://raw.githubusercontent.com/Xqhare/ananke/refs/heads/master/pictures/logo.jpeg" alt="Ananke logo. Golden symbols, loosely ressembling hyroglyphs.", width=auto, height="250">
</p>

It follows my "All code written by me or part of rust's standard library and libc" philosophy.
You can learn more about that [here](https://blog.xqhare.net/posts/why_solve_problems/).

Ananke also serves as a demo for my code stack, pulling in 6 projects of mine directly.

## Features
- _**No dependencies**_: All code is written by me or part of std.
- Cross-platform: Linux & macOS
- Standalone executable
- Simple, no frills UI
- Implements the [todo.txt](https://github.com/todotxt/todo.txt) specification
- Supports multiple todo files
- Supports sorting by date, priority, or inception and completion date
- Supports search by date, priority, or text and tags

## Known issues

- Crashes if height is less than 20 lines
- Tasks may not render correctly if starting outside the viewport and scrolled to. This only happens with some terminal heights, especially during resizing.
  - To fix, resize the terminal to a higher height and restart.

## Getting started

To install, you will probably need to compile from source:

- You will need to have [Rust](https://www.rust-lang.org) installed. Use the [rustup](https://rustup.rs/) tool to install it.
- Clone the repository.
- Run `cargo build --release`.
- Move the binary (from `target/release/ananke`) to a directory of your choice and run.

### Alternative

I provide the binary compiled on my system (debian 13) in the [releases](https://github.com/Xqhare/ananke/releases). \
If you are lucky it works!

### Windows

If you really want to use this on Windows, use `WSL` ([learn more](https://learn.microsoft.com/en-us/windows/wsl/install)) and compile it. Should work, but I have not tested it.

### Usage

Ananke is a todo.txt interface, so you can use it to manage your tasks.

To interact with Ananke, use your mouse and click or scroll.

Keyboard shortcuts:

- `q` to quit
- `esc` to loose `focus` (keyboard capture for text entry)

This is the main screen:

<p align="center">
<img src="https://raw.githubusercontent.com/Xqhare/ananke/refs/heads/master/pictures/main_img.png" alt="Main screen of Ananke.", width=auto, height="300">
</p>

On top, there are 4 Buttons:

- File
  - Load a todo file
- Save
  - Save the current todo file; Ananke automatically saves when you change anything, but you can also save manually to feel better.
- Help
  - Open the help screen
- Exit
  - Exits

Below is a large field to enter a new task.

Again below is a menu for sorting and searching the list of tasks.

The list of tasks is rendered on the remaining area of the screen.

### Interactivity

Some UI elements are interactive.

If you can enter text, the element is drawn with a double border.

<p align="center">
<img src="https://raw.githubusercontent.com/Xqhare/ananke/refs/heads/master/pictures/interact_img.png" alt="Example of interactivity within Ananke.", width=auto, height="300">
</p>

## Name

As with all my projects, Ananke is named after an ancient deity.
Learn more about my naming scheme [here](https://blog.xqhare.net/posts/explaining_the_pantheon/).

Ananke is the ancient Greek goddess embodying destiny and the natural order of things. She is also associated with fulfilling ones obligations.

### Ananke Never Abandons Needs Known Early
The name is also a reverse acronym.

## Technical details
Inside this repository you will find only the frontend code.
The backend for `Ananke` is `Anansi`, a todo.txt interface written by me - You can find it [here](https://github.com/Xqhare/anansi).
