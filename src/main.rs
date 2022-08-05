#[macro_use]
extern crate log;

mod editor_core;
mod editor;

use editor_core::{cli_handler, file_handler::FileHandler};
use editor::custom::CustomEditorState;


fn main() {

    // If cli_parser returns filename then populate the buffer
    let mut app = if let Some(file) = cli_handler::parse() {
        debug!("printing file {:?}", file);
        let mut fd = FileHandler::from(file);
        let buffer = fd.read_lined_buffer();

        CustomEditorState::from(buffer, fd)
    } else {
        // open TUI with empty buffer
        CustomEditorState::new()
    };


    let _result = app.run();
}
