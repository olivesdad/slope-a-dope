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
    currently_editing: Option<CurrentlyEditing>,
    temp_point: (String, String),
}

pub struct TempPoint {
    pub input_p: Option<String>,
    pub input_v: Option<String>,
}
pub enum CurrentlyEditing {
    Voltage,
    Physical,
}

pub enum Mode {
    Edit,
    Select,
    Quit,
    EditPoint,
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
            currently_editing: None,
            temp_point: (String::new(), String::new()),
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
    pub fn get_line_val(&self) -> String{
        if let Some(line) = &self.line {
            if let Some(vals) = line.get_val() {
                format!("Slope: {:.4} Intercept: {:.4}", vals.0 ,vals.1)
            } else {
                "Unable to calculate line".to_owned()
            }
        } else {
            "Unable to calculate line".to_owned()
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
            Mode::EditPoint => self.edit_point()?,
            _=>{}
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
                KeyCode::Enter => {
                    self.mode = Mode::EditPoint;
                }
                _ => {}
            }
        }
        Ok(())
    }


    // edit a point
    fn edit_point(&mut self) -> Result<(),()>{

        // If were already editing then we need to match the state
        if let Some(current) = &self.currently_editing {
            //listen for a keypress 
            if let Some(key) = get_key_press() {
                // determine which point were editing
                match current {
                    CurrentlyEditing::Voltage => {
                        // match the keypress
                        match key {
                            KeyCode::Char(c) => {
                                self.temp_point.0.push(c);
                            }
                            KeyCode::Enter =>{
                                // If we press enter then change currently editing to Physical
                                self.currently_editing = Some(CurrentlyEditing::Physical)
                            }
                            KeyCode::Backspace => {
                                self.temp_point.0.pop();
                            }
                            _=>{}
                        }
                    }
                    CurrentlyEditing::Physical => {
                        match key {
                            KeyCode::Backspace => {
                                self.temp_point.1.pop();
                            }
                            KeyCode::Char(c) => {
                                self.temp_point.1.push(c);
                            }
                            KeyCode::Enter =>{
                                // If we press enter then change currently editing to None, clear Strings, convert strings into new point
                                self.currently_editing = None;
                                self.mode = Mode::Select;
                                
                                //try to parse points
                                let v = self.temp_point.0.parse::<f64>().unwrap_or(0.0);
                                let p = self.temp_point.1.parse::<f64>().unwrap_or(0.0);
                                // a new point
                                let point = Point::from((v,p));
                                //wipe string
                                self.temp_point = (String::new(), String::new());
                                // assign the new point to p1 or p2 and then update line
                                match self.current_screen {
                                    ScreenID::P1 =>{
                                        self.p1 = Some(point);
                                        self.update_line();
                                    }
                                    ScreenID::P2 => {
                                        self.p2 = Some(point);
                                        self.update_line();
                                    }
                                    _=>{}
                                }
                            }
                            _=>{}
                        }
                    }
                }
            }
        } else {
            self.currently_editing = Some(CurrentlyEditing::Voltage);
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
