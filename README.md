# Ananke

Ananke is a fully featured to-do app written in Rust for all UNIX platforms.

<p align="center">
<img src="https://raw.githubusercontent.com/Xqhare/ananke/refs/heads/master/pictures/logo.jpeg" alt="Ananke logo. Golden symbols, loosely ressembling hyroglyphs.", width=auto, height="450">
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

## Name

As with all my projects, Ananke is named after an ancient deity.
Learn more about my naming scheme [here](https://blog.xqhare.net/posts/explaining_the_pantheon/).

Ananke is the ancient Greek goddess embodying destiny and the natural order of things. She is also associated with fulfilling ones obligations.

### Ananke Never Abandons Needs Known Early
The name is also a reverse acronym.

## Technical details
Inside this repository you will find only the frontend code.
The backend for `Ananke` is `Anansi`, a todo.txt interface written by me - You can find it [here](https://github.com/Xqhare/anansi).
