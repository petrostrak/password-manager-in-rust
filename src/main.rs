use std::error::Error;

use crossterm::event::Event::Key;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use tui::backend::{Backend, CrosstermBackend};
use tui::text::Span;
use tui::widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph};
use tui::{Frame, Terminal};

const APP_KEYS_DESC: &str = r#"
L:           List
U:           On list, It's copy the Username
P:           On list, It's copy the Password
D:           On list, It's Delete
E:           On list, It's Edit
S:           Search
Insert Btn:  Insert new Password
Tab:         Go to next field
Shift+Tab:   Go to previous filed
Esc:         Exit insert mode
"#;

enum InputMode {
    Normal,
    Title,
    Username,
    Password,
    Submit,
    Search,
    List,
}

struct Password {
    title: String,
    username: String,
    password: String,
}

struct PassMng {
    mode: InputMode,
    list_state: ListState,
    passwords: Vec<Password>,
    search_text: String,
    search_list: Vec<Password>,
    new_title: String,
    new_username: String,
    new_password: String,
}

impl PassMng {
    pub fn new() -> Self {
        PassMng {
            mode: InputMode::Normal,
            list_state: ListState::default(),
            passwords: vec![],
            search_text: String::new(),
            search_list: vec![],
            new_title: String::new(),
            new_username: String::new(),
            new_password: String::new(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut state = PassMng::new();

    enable_raw_mode()?;
    execute!(std::io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal, &mut state);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;

    if let Err(e) = result {
        println!("{}", e.to_string());
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    state: &mut PassMng,
) -> Result<(), std::io::Error> {
    loop {
        terminal.draw(|f| ui(f, state))?;

        if let Key(key) = event::read()? {
            match state.mode {
                InputMode::Normal => {
                    match key.code {
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        KeyCode::Char('s') => {
                            // search mode
                        }
                        KeyCode::Char('l') => {
                            // list mode
                        }
                        KeyCode::Insert => {
                            // insert mode
                        }
                        _ => {}
                    }
                }

                InputMode::Title => {
                    match key.code {
                        KeyCode::Esc => {
                            // exit from Title to Normal mode
                        }
                        KeyCode::Char(c) => state.new_title.push(c),
                        KeyCode::Backspace => {
                            state.new_title.pop();
                        }
                        _ => {}
                    }
                }

                InputMode::Username => {
                    match key.code {
                        KeyCode::Esc => {
                            // exit from Title to Normal mode
                        }
                        KeyCode::Char(c) => state.new_username.push(c),
                        KeyCode::Backspace => {
                            state.new_username.pop();
                        }
                        _ => {}
                    }
                }

                InputMode::Password => {
                    match key.code {
                        KeyCode::Esc => {
                            // exit from Title to Normal mode
                        }
                        KeyCode::Char(c) => state.new_password.push(c),
                        KeyCode::Backspace => {
                            state.new_password.pop();
                        }
                        _ => {}
                    }
                }

                InputMode::Submit => {
                    match key.code {
                        KeyCode::Esc => {
                            // exit from Title to Normal mode
                        }
                        KeyCode::BackTab => {
                            // change mode to Password
                        }
                        _ => {}
                    }
                }

                InputMode::Search => {
                    match key.code {
                        KeyCode::Esc => {
                            // exit from Title to Normal mode
                        }
                        _ => {}
                    }
                }

                InputMode::List => {
                    match key.code {
                        KeyCode::Esc => {
                            // exit from Title to Normal mode
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, state: &mut PassMng) {}
