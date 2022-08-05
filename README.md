# What is RSVIM?

Its a Vim like modal Text editor, made with rust. Key points which make RSVIM Stand out are:

- `.toml` Configs: Editing `.*rc` files can be tedious, hence RSVIM provides easier way to modify and change your text editor.
- Its fast

Built upon [tui-rs](https://github.com/fdehau/tui-rs)

First Phase of the release include text input
***

## Usage

You can execute this binary by running `cargo run`, this will start the editor with empty buffer. Inorder to open a file, pass the file path to the binary. Use this command as an example `cargo run -- -f ./test.txt`.

Note: Editing mode is WIP

***

## Features [TODO]

- [X]  VIM Key bindings movement
- [ ]  text wrapping
- [ ]  Synxtax highlighting
- [ ]  Undo/Redo
- [ ]  Custom Keybindings
