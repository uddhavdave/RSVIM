#[macro_use]
extern crate log;

mod editor_core;
mod editor;

use editor_core::cli_handler;
use editor::custom::CustomEditorState;
use std::fs::File;
use std::fs;
use std::io::Read;

fn main() {

    // If cli_parser returns filename then populate the buffer
    let mut app = if let Some(file) = cli_handler::parse() {
        debug!("printing file {:?}", file);
        // populate a buffer and pass it to TUI
        let mut f = File::open(&file).expect("no file found");
        let metadata = fs::metadata(&file).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");
        // TODO: Split the buffer at 'LF' and handle cases where file is edited with 'CRLF' pattern
        CustomEditorState::from(buffer)
    } else {
        // open TUI with empty buffer
        CustomEditorState::new()
    };


    let _result = app.run();
}
