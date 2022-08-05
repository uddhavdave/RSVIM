use crate::editor_core::file_handler::FileHandler;

use super::*;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute, 
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::str;

enum InputMode {
    Normal,
    Editing,
}

const HORIZONTAL_OFFSET : u16 = 6;
const VERTICAL_OFFSET : u16 = 1;
/// Keeps track of editor state
pub(crate) struct CustomEditorState {
    /// User Input 
    input: Vec<u8>,
    /// Text Editor Mode
    input_mode: InputMode,
    /// Display Buffer
    buffer: Vec<Vec<u8>>,
    /// Cursor position relative to Block
    cursor_line_position: (usize, usize),
    /// File Read Write handler
    file_handler: Option<FileHandler>,
}

/// Sets the initial state of the editor 
impl Default for CustomEditorState {
    fn default() -> CustomEditorState {
        CustomEditorState {
            input: Vec::new(),
            /// Always start editor in viewing mode
            input_mode: InputMode::Normal,
            buffer: Vec::new(),
            cursor_line_position: ( 0 , 0 ),
            file_handler: None,
        }
    }
}

impl CustomEditorState {
    pub fn new() -> Self {
        CustomEditorState::default()
    }

    pub fn from(buffer: Vec<Vec<u8>>, fd: FileHandler) -> Self {
        CustomEditorState {
            buffer: buffer,
            file_handler: Some(fd),
            ..Default::default()
        }
    }
    
    // ! this is a blocking function
    pub fn run(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let res = run_app(&mut terminal, self);

        // restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        if let Err(err) = res {
            println!("{:?}", err)
        }

        Ok(())
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut CustomEditorState) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;

                        // Read the current line from buffer
                        app.input = app.buffer[app.cursor_line_position.1].clone();
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Char('l') => {
                        if app.cursor_line_position.0 < app.buffer[app.cursor_line_position.1].len() - 1 {
                            app.cursor_line_position = (app.cursor_line_position.0 + 1, app.cursor_line_position.1);
                        }
                    }
                    KeyCode::Char('h') => {
                        if app.cursor_line_position.0 > 0 {
                            app.cursor_line_position = (app.cursor_line_position.0 - 1, app.cursor_line_position.1);
                        }
                    }
                    KeyCode::Char('j') => {
                        if app.cursor_line_position.1 < app.buffer.len() - 1 {
                            app.cursor_line_position = (app.cursor_line_position.0 , app.cursor_line_position.1 + 1);
                        }
                    }
                    KeyCode::Char('k') => {
                        if app.cursor_line_position.1 > 0 {
                            app.cursor_line_position = (app.cursor_line_position.0 , app.cursor_line_position.1 - 1);
                        }
                    }
                    KeyCode::Char('w') => {
                        // Save the file
                        if let Some(file_handler) = &app.file_handler {
                            file_handler.write_lined_buffer(app.buffer.clone())?;
                        } else {
                            // Ask for filename and save 
                        }
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        // when Enter key is pressed, the line will be split at the current position
                        let (prev_line, next_line) = app.input.split_at(app.cursor_line_position.0);

                        // Update the previous line in the buffer
                        app.buffer[app.cursor_line_position.1].clear();
                        app.buffer[app.cursor_line_position.1] = prev_line.to_vec();
                        
                        // Add next line to the buffer 
                        app.cursor_line_position = ( 0, app.cursor_line_position.1 + 1);
                        app.buffer.insert(app.cursor_line_position.1, Vec::new());
                        
                        // Update the newly inserted line
                        app.buffer[app.cursor_line_position.1].clear();
                        app.buffer[app.cursor_line_position.1] = next_line.to_vec();

                        // Change current line buffer, set it to next line
                        app.input = next_line.to_vec();
                    }
                    KeyCode::Char(c) => {
                        app.input.insert(app.cursor_line_position.0, c as u8);
                        app.cursor_line_position = ( app.cursor_line_position.0 + 1, app.cursor_line_position.1);
                        app.buffer[app.cursor_line_position.1] = app.input.clone();
                    }
                    KeyCode::Backspace => {
                        app.input.remove(app.cursor_line_position.0);
                        app.cursor_line_position = ( app.cursor_line_position.0 - 1, app.cursor_line_position.1);
                        app.buffer[app.cursor_line_position.1] = app.input.clone();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &CustomEditorState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing,"),
                Span::styled("w", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to save."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the message"),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[1]);

    // let input = Paragraph::new(app.input.as_ref())
    //     .style(match app.input_mode {
    //         InputMode::Normal => Style::default(),
    //         InputMode::Editing => Style::default().fg(Color::Yellow),
    //     })
    //     .block(Block::default().borders(Borders::ALL).title("Input"));
    // f.render_widget(input, chunks[1]);

    match app.input_mode {
        InputMode::Normal => {
            // Move the cursor with VIM Key Strokes
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[0].x + app.cursor_line_position.0 as u16 + HORIZONTAL_OFFSET,
                // Move one line down, from the border to the input line
                chunks[0].y + app.cursor_line_position.1 as u16 + VERTICAL_OFFSET,
            )
        }

        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[0].x + app.cursor_line_position.0 as u16 + HORIZONTAL_OFFSET,
                // Move one line down, from the border to the input line
                chunks[0].y + app.cursor_line_position.1 as u16 + VERTICAL_OFFSET,
            )
        }
    }

    let lines = app.buffer.iter().enumerate().map(|(line_number, line)| match str::from_utf8(&line) {
        Ok(v) => format!("{:4} {}", line_number, v),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    }).collect::<Vec<String>>().join("\n");
    let para =
        Paragraph::new(lines).block(Block::default().borders(Borders::ALL));
    f.render_widget(para, chunks[0]);
}

