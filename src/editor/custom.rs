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
use unicode_width::UnicodeWidthStr;
use std::str;

enum InputMode {
    Normal,
    Editing,
}

/// Keeps track of editor state
pub(crate) struct CustomEditorState {
    /// User Input 
    input: String,
    /// Text Editor Mode
    input_mode: InputMode,
    /// Display Buffer
    buffer: Vec<u8>,
    /// X Coordinate of Cursor
    cursor_x_pos: usize,
    /// Y Coordinate of Cursor 
    cursor_y_pos: usize,
}

/// Sets the initial state of the editor 
impl Default for CustomEditorState {
    fn default() -> CustomEditorState {
        CustomEditorState {
            input: String::new(),
            /// Always start editor in viewing mode
            input_mode: InputMode::Normal,
            buffer: Vec::new(),
            cursor_x_pos: 0,
            cursor_y_pos: 0,
        }
    }
}

impl CustomEditorState {
    pub fn new() -> Self {
        CustomEditorState::default()
    }

    pub fn from(buffer: Vec<u8>) -> Self {
        CustomEditorState {
            buffer: buffer,
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
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Char('l') => {
                        app.cursor_x_pos += 1;
                    }
                    KeyCode::Char('h') => {
                        if app.cursor_x_pos > 0 {
                            app.cursor_x_pos -= 1;
                        }
                    }
                    KeyCode::Char('j') => {
                        app.cursor_y_pos += 1;
                    }
                    KeyCode::Char('k') => {
                        if app.cursor_y_pos > 0 {
                            app.cursor_y_pos -= 1;
                        }
                    }
                    KeyCode::Char('w') => {
                        // Save the file
                        todo!();
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        for char in app.input.as_bytes() {
                            app.buffer.push(*char);
                        }
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
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
                Constraint::Length(1),
                Constraint::Min(1),
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
                Span::raw(" to start editing."),
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
    f.render_widget(help_message, chunks[0]);

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
                chunks[1].x + app.cursor_x_pos as u16,
                // Move one line down, from the border to the input line
                chunks[1].y + app.cursor_y_pos as u16,
            )
        }

        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[1].x + app.input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[1].y + 1,
            )
        }
    }
    let messages = match str::from_utf8(&app.buffer) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let messages =
        Paragraph::new(messages).block(Block::default().borders(Borders::ALL));
    f.render_widget(messages, chunks[1]);
}
