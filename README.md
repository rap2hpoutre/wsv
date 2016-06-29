# wsv
WSV (Whathever Separated Values) converts a so-called CSV-file to another so-called CSV-file. It just replaces the separator: `,` to `;` or `;` to `|` or `rainbow` to `poney`. It takes care to separator inside delimiter `"` and does not replace it. This project is useless, I'm just learning Rust.

## Usage
```
Whathever Separated Values
Replace comma (or something else) by semicolon (or something else) in a so-called CSV.

Usage:
  wsv [-s <s>] [-d <d>] <source> <dest>
  wsv (-h | --help)

Options:
  -s <s>        Source separator [default: ,]
  -d <d>        Destination separator [default: ;]
  -h --help     Show this screen.
```
