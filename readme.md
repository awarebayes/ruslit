# Translit

![Image](https://private-user-images.githubusercontent.com/42784580/382023556-ceb933c5-b418-4133-bf78-b197e39e49c4.png?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3MzAzOTEzOTEsIm5iZiI6MTczMDM5MTA5MSwicGF0aCI6Ii80Mjc4NDU4MC8zODIwMjM1NTYtY2ViOTMzYzUtYjQxOC00MTMzLWJmNzgtYjE5N2UzOWU0OWM0LnBuZz9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNDEwMzElMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjQxMDMxVDE2MTEzMVomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPTAwZTEzNjdlZDU2ODFkOTg4MGMxNDk5OGE3Y2YzNjkwNGRhYTAzYzhjYWVhNmQyNWM3YjI4NjEwNjYyMjQyYTYmWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0In0.HN4SH1TM9MP_RmZjfNPvaDez8pv3rCOYCN2gS6F17qo)

## Rinning

`cargo build --release`

`./rust_translit --clipboard xclip`

## Binds:

Esc - exit

Return - copy to clipboard and clear input

Supported clipboards: "xclip", "arboard", "wlclipboard"

Supported languages: Russian. You can add your own (Urkainian, Belarussian) via forking and PR. Dont forget to add cli option.

GOST Changes:

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
