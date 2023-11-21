use std::collections::HashMap;

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
        let mut app = App {
            p1: Some(Point::from((5.0, 100.0))),
            p2: Some(Point::from((0.0, 0.0))),
            line: None,
            current_screen: ScreenID::P1,
            mode: Mode::Select,
        };
        app.update_line();
        return app;
    }

    //calculates the line based on the 2 points if one of the points is NONE then
    pub fn update_line(&mut self) {
        if let Some(p1) = &self.p1 {
            if let Some(p2) = &self.p2 {
                self.line = Some(Line::from((p1, p2)));
            } else {
                self.line = None;
            }
        } else {
            self.line = None;
        }
    }
    // Get tuple with (m,b) from line
    pub fn get_line_val(&self) -> Option<(f64, f64)> {
        if self.line.is_some() {
            self.line.as_ref().unwrap().get_val()
        } else {
            return None;
        }
    }
    // track the current screen
    pub fn get_current_screen(&self) -> &ScreenID {
        &self.current_screen
    }

    //get Point values
    pub fn get_points(&self) -> Option<(HashMap<&str, f64>, HashMap<&str, f64>)> {
        if let Some(p1) = &self.p1 {
            if let Some(p2) = &self.p2 {
                return Some((p1.get_val(), p2.get_val()));
            } else {
                return None;
            }
        } else {
            return None;
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
            let screen = self.get_current_screen();
            match key {
                KeyCode::Esc => {
                    self.mode = Mode::Quit;
                }
                KeyCode::Left => match screen {
                    ScreenID::Tester => {
                        self.current_screen = ScreenID::P1;
                    }
                    _ => {}
                },
                KeyCode::Right => match screen {
                    ScreenID::P1 | ScreenID::P2 => {
                        self.current_screen = ScreenID::Tester;
                    }
                    _ => {}
                },
                KeyCode::Up => match screen {
                    ScreenID::P2 => {
                        self.current_screen = ScreenID::P1;
                    }
                    _ => {}
                },
                KeyCode::Down => match screen {
                    ScreenID::P1 => {
                        self.current_screen = ScreenID::P2;
                    }
                    _ => {}
                },
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
