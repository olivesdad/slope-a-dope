use crossterm::event;
use crossterm::event::{Event, KeyCode};

use crate::calculator::{Line, Point};

pub struct App {
    p1: Option<Point>,
    p2: Option<Point>,
    line: Option<Line>,
    current_screen: ScreenID,
    mode: Mode,
}

pub enum Mode {
    Edit,
    Select,
    Quit,
}
pub enum ScreenID {
    P1,
    P2,
    Tester,
    Quit,
}

impl App {
    pub fn new() -> Self {
        App {
            p1: None,
            p2: None,
            line: None,
            current_screen: ScreenID::P1,
            mode: Mode::Select,
        }
    }

    pub fn get_mode(&self) -> &Mode {
        &self.mode
    }

    // Update the status we should call differnt functions based on the modes
    pub fn update_state(&mut self) -> Result<(), ()> {
        match self.mode {
            Mode::Edit => self.update_editor_mode()?,
            Mode::Select => self.update_selector_mode()?,
            Mode::Quit => return Err(()),
        }
        Ok(())
    }

    //fn for editor mode
    fn update_editor_mode(&mut self) -> Result<(), ()> {
        if let Some(key) = get_key_press() {
            match key {
                KeyCode::Esc => {
                    self.mode = Mode::Select;
                }
                _ => {}
            }
        }
        Ok(())
    }

    //fn for editor mode
    fn update_selector_mode(&mut self) -> Result<(), ()> {
        if let Some(key) = get_key_press() {
            match key {
                KeyCode::Esc => {
                    self.mode = Mode::Quit;
                }
                KeyCode::Left => {
                    //move selector left
                }
                KeyCode::Right => {
                    // move selector right
                }
                KeyCode::Up => {
                    // move selector up
                }
                KeyCode::Down => {
                    // move selector down
                }
                KeyCode::Enter => {
                    // change to editor mode
                    self.mode = Mode::Edit;
                }
                // Every other one is useless
                _ => {}
            }
        }

        Ok(())
    }
}

// Function to get key press. We will just sit in the loop until we get a keypress
fn get_key_press() -> Option<KeyCode> {
    loop {
        // Read a key
        let e = event::read();
        if e.is_ok() {
            if let Event::Key(key) = e.unwrap() {
                if key.kind == event::KeyEventKind::Release {
                    // Skip events that are not KeyEventKind::Press
                    continue;
                } else {
                    return Some(key.code);
                }
            } else {
                return None;
            }
        }
    }
}
