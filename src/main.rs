#[macro_use]
extern crate log;

mod editor_core;
mod editor;

use editor_core::{cli_handler, file_handler::FileHandler};
use editor::custom::CustomEditorState;

fn main() {

    // Check for filepath in CLI arguments
    let mut app = if let Some(file) = cli_handler::parse() {
        debug!("file {:?}", file);
        let mut fd = FileHandler::from(file);
        let buffer = fd.read_lined_buffer();
        // Open Editor with File contents
        CustomEditorState::from(buffer, fd)
    } else {
        // Open Editor with empty buffer
        CustomEditorState::new()
    };
    // Blocking Call
    let _result = app.run();
}
