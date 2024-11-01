# Translit

<img width="712" alt="image" src="https://github.com/user-attachments/assets/6be754d5-1db9-4e57-a7a2-4ad94557f7be">

## Rinning

`cargo build --release`

`./rust_translit --clipboard xclip --language by`

## Options

Supported clipboards: `arboard` (default), `xclip`, `wlclipboard`.

Supported platforms: X11, Wayland, whatever `arboard` crate supports.

Supported languages: `ru` (default), `ua`, `by`.

## Binds:

Esc - exit

Return - copy to clipboard and clear input

## Misc

Russian GOST Changes:

```rust
table.extend(vec![
    ("ь", "'"),
    ("ъ", "''"),
    ("ы", "y'"),
    ("Ы", "Y'"),
    ("в", "w"),
    ("В", "W"),
    ("э", "e'"),
    ("Э", "E'"),
    ("Х", "H"),
    ("х", "h"),
]);
```

### Why did I make it

I use corne keyboard with colemak-dh english layout programmed via VIAL (with custom binds).

For the love of god I do not want to learn to type russian in colemak.
