pub use std::io;

// pub enum EditorError {
//     WrongFilePath,
//     FileNotSaved,
// }

//TODO Add trait for Apps running Backend
// Note: This Trait allows users to implement their own control logic (for example switching from Vim Key Settings to Emacs Key Settings) 
// 
// This will handle the 
// pub trait TuiEngine {
//     // ! This method is blocking in nature 
//     fn run_custom_mappings<B: Backend>(&mut self, terminal: Terminal) -> Result<(), EditorError>;
//     fn close_ui() -> Result<(), io::Result<()>>;
// }

pub mod custom;