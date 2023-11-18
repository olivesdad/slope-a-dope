use crate::calculator::{Line, Point};

pub struct App {
    p1: Option<Point>,
    p2: Option<Point>,
    line: Option<Line>,
    current_screen: ScreenID,
    selector: ScreenID,
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
            selector: ScreenID::P1,
        }
    }
}
