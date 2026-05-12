# Ananke
Ananke is a fully featured to-do app written in Rust for all PC platforms.

## Features
- Cross-platform
- Minimal dependencies

## TODO

## Known issues

- Crashes if height is less than 20 lines
- Tasks may not render correctly if starting outside the viewport and scrolled to. This only happens with some terminal heights, especially during resizing.
  - To fix, resize the terminal to a higher height and restart.

## Name
Ananke is named after the Greek goddess embodying destiny and the natural order of things. She is also associated with fulfilling ones obligations.

### Ananke Never Abandons Needs Known Early
The name is also a reverse acronym.

## Technical details
Inside this repository you will find only the frontend code.
The backend for `Ananke` is `Anansi`, a todo.txt interface written by me - You can find it [here](https://github.com/Xqhare/anansi).
