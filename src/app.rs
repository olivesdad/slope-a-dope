use std::collections::HashMap;

use crossterm::event;
use crossterm::event::{Event, KeyCode};

use crate::calculator::{Line, MeasurementType, Point};

pub struct App {
    p1: Option<Point>,
    p2: Option<Point>,
    line: Option<Line>,
    current_screen: ScreenID,
    mode: Mode,
    currently_editing: Option<CurrentlyEditing>,
    temp_point: Option<String>,
    plot: Vec<(f64, f64)>,
}

pub enum CurrentlyEditing {
    Voltage,
    Physical,
}

pub enum Mode {
    Edit,
    EditingValue,
    Select,
    Quit,
}
pub enum ScreenID {
    P1,
    P2,
    Tester,
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
            temp_point: None,
            plot: Vec::new(),
        };
        app.update_line();
        return app;
    }

    //calculates the line based on the 2 points if one of the points is NONE then
    pub fn update_line(&mut self) {
        if let Some(p1) = &self.p1 {
            if let Some(p2) = &self.p2 {
                self.line = Some(Line::from((p1, p2)));
                self.update_vector();
            } else {
                self.line = None;
            }
        } else {
            self.line = None;
        }
    }

    // Function to update the vector we will use to plot
    pub fn update_vector(&mut self) {
        // vector contents should look like [(v0,p0),(v1,p1), (v2,p2)....(vn,pn)]

        // Wipe the existing vector
        self.plot = Vec::new();

        // generate new vector
        // get the highest x and y value

        // get the lowest x and y value
        let start: MeasurementType;
        let end: MeasurementType;
        if let (Some(x1), Some(x2)) = (self.p1.as_ref(), self.p2.as_ref()) {
            let tmp1 = x1.get_val().get("v").unwrap_or(&0.0).clone();
            let tmp2 = x2.get_val().get("v").unwrap_or(&0.0).clone();
            if tmp1 < tmp2 {
                start = MeasurementType::voltage(tmp1.clone());
                end = MeasurementType::voltage(tmp2.clone());
            } else {
                start = MeasurementType::voltage(tmp2.clone());
                end = MeasurementType::voltage(tmp1.clone());
            }
            if let Some(l) = self.line.as_ref() {
                let _res1 = self
                    .plot
                    .push((tmp1, l.get_corresponding_value(start).unwrap()));
                let _res2 = self
                    .plot
                    .push((tmp2, l.get_corresponding_value(end).unwrap()));
            }
        }
    }

    /*
     -=-=-=-=-=-  Getters and Setters -=-=-=-=-=-=-
    */

    pub fn get_test_point(&self, v: MeasurementType) -> Result<f64, ()> {
        let line = self.line.as_ref().ok_or(())?;
        line.get_corresponding_value(v)
    }
    // Get tuple with (m,b) from line
    pub fn get_line_val(&self) -> String {
        if let Some(line) = &self.line {
            if let Some(vals) = line.get_val() {
                format!("Slope: {:.4} Intercept: {:.4}", vals.0, vals.1)
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
    pub fn get_temp_point(&self) -> &str {
        match self.temp_point.as_ref() {
            Some(x) => x,
            None => "",
        }
    }
    pub fn get_currently_editing(&self) -> &Option<CurrentlyEditing> {
        &self.currently_editing
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

    /*
     *  App control functions. The App is drivern by main by calling update_state.
     *  Update state will use different control flow depending on the curren mode
     */

    // Update the status we should call differnt functions based on the modes
    pub fn update_state(&mut self) -> Result<(), ()> {
        match self.mode {
            Mode::Edit => self.update_editor_mode()?,
            Mode::Select => self.update_selector_mode()?,
            Mode::Quit => return Err(()),
            Mode::EditingValue => self.edit_value()?,
        }
        Ok(())
    }

    /*
     * The following functions are called by update_state depending on the mode. They will listen for keypress
     * by calling the get_key_press function  then handle the keypress accodringly
     */

    /*
     * MODE = EditingValue
     * In this Mode we want to just edit a string to display in place of the value
     * We want the user to be able to edit 1 value from each point
     * the UI must handle switching the displayed text from the actual point to this.
     * w
     */
    fn edit_value(&mut self) -> Result<(), ()> {
        // Access String in temp_point
        if let Some(s) = self.temp_point.as_mut() {
            //listen for keypress
            if let Some(key) = get_key_press() {
                match key {
                    KeyCode::Backspace => {
                        s.pop();
                    }
                    KeyCode::Char(c) => {
                        s.push(c);
                    }
                    KeyCode::Esc => {
                        // escape will clear the string and switch mode back to editing
                        self.temp_point = None;
                        self.mode = Mode::Edit;
                    }
                    KeyCode::Enter => {
                        // Enter will attempt to push the value back into the point. If it fails to parse the value change nothing
                        // Recalculate line if that succeeeds

                        // I need to know: which point and which value of that point
                        // Which point were editing
                        let mut point_ref = None;
                        match self.current_screen {
                            ScreenID::P1 => {
                                point_ref = self.p1.as_mut();
                            }
                            ScreenID::P2 => {
                                point_ref = self.p2.as_mut();
                            }

                            _ => {} // TODO: Impliment tester
                        }
                        // Which value of the point
                        if let Some(ce) = self.currently_editing.as_ref() {
                            // Try to parse the string to f64. If it's succesful update the point
                            if let Some(val) = self.temp_point.as_ref() {
                                if let Ok(parsed) = val.parse::<f64>() {
                                    match ce {
                                        CurrentlyEditing::Physical => {
                                            if let Some(p) = point_ref {
                                                p.set_physical(parsed);
                                            }
                                        }
                                        CurrentlyEditing::Voltage => {
                                            if let Some(p) = point_ref {
                                                p.set_voltage(parsed);
                                            }
                                        }
                                    }
                                    // Recalculate the line
                                    self.update_line();
                                }
                            }
                        }
                        // Wipe the temp string back to None
                        self.temp_point = None;
                        // Switch the mode back
                        self.mode = Mode::Edit;
                    }

                    _ => {}
                }
            }
        } else {
            self.temp_point = Some(String::new());
        }
        Ok(())
    }

    /*
     * Mode = Edit
     * This function is entered when we are in Editing mode. The name is kind of missleading because of the initial design. It is more like "pre-edit"
     * It needs to:
     *  - If escape is pressed it needs to: { turn currently_editing back to None, switch mode to select}
     *  - set currently_editing enum to Some(CurrentlyEditing)
     *  - Toggle which value, physical or voltage, is being currently edited as arrow keys are pressed
     *  - enter EditingValue mode if enter is pressed
     */
    fn update_editor_mode(&mut self) -> Result<(), ()> {
        // check if we have a currently_editing value
        if self.currently_editing.is_some() {
            if let Some(key) = get_key_press() {
                match key {
                    KeyCode::Esc => {
                        self.currently_editing = None;
                        self.mode = Mode::Select;
                    }
                    KeyCode::Enter => {
                        self.mode = Mode::EditingValue;
                    }
                    KeyCode::Down => match self.current_screen {
                        ScreenID::P1 | ScreenID::P2 => {
                            self.currently_editing = Some(CurrentlyEditing::Physical);
                        }
                        _ => {}
                    },
                    KeyCode::Up => match self.current_screen {
                        ScreenID::P1 | ScreenID::P2 => {
                            self.currently_editing = Some(CurrentlyEditing::Voltage);
                        }
                        _ => {}
                    },
                    KeyCode::Left => match self.current_screen {
                        ScreenID::Tester => {
                            self.currently_editing = Some(CurrentlyEditing::Voltage);
                        }
                        _ => {}
                    },
                    KeyCode::Right => match self.current_screen {
                        ScreenID::Tester => {
                            self.currently_editing = Some(CurrentlyEditing::Physical)
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        } else {
            self.currently_editing = Some(CurrentlyEditing::Voltage);
        }
        Ok(())
    }

    /*
     * MODE = Select
     * This mode is just responsible for switching the screen, quitting, and entering Edit mode
     */
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
                    self.currently_editing = Some(CurrentlyEditing::Voltage);
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
