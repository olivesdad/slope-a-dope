pub struct Line {
    slope: Option<f64>,
    intercept: Option<f64>,
}
pub struct Point {
    voltage: Option<f64>,
    physical: Option<f64>,
}

impl Point {
    pub fn new() -> Self {
        Point {
            voltage: None,
            physical: None,
        }
    }
    pub fn is_valid(&self) -> bool {
        if self.voltage.is_some() && self.physical.is_some() {
            true
        } else {
            false
        }
    }
    pub fn set_points(&mut self, x: f64, y: f64) {
        self.voltage = Some(x);
        self.physical = Some(y);
    }
}

impl Line {
    pub fn new() -> Self {
        Line {
            slope: None,
            intercept: None,
        }
    }

    pub fn calc(&mut self, p1: &Point, p2: &Point) {
        if p1.is_valid() && p2.is_valid() {
            if p1.voltage == p2.voltage {
                self.slope = None;
                self.intercept = None;
            } else {
                let m = (&p1.physical.unwrap() - &p2.physical.unwrap())
                    / (&p1.voltage.unwrap() - &p2.voltage.unwrap());
                let b = -(&m * p1.voltage.unwrap()) + p1.physical.unwrap();
                self.slope = Some(m);
                self.intercept = Some(b);
            }
        } else {
            self.slope = None;
            self.intercept = None;
        }
    }
}

impl From<(&Point, &Point)> for Line {
    fn from(points: (&Point, &Point)) -> Self {
        let mut line = Line::new();
        line.calc(points.0, points.1);
        line
    }
}

// -------TESTS --------
#[cfg(test)]
mod tests {
    use super::{Line, Point};

    #[test]
    fn create_points() {
        let mut p1 = Point::new();
        let mut p2 = Point::new();
        let mut line = Line::new();
        line.calc(&p1, &p2);
        // This should fail because points have no values
        assert_eq!(None, line.slope);
        assert_eq!(None, line.intercept);

        //set values for p1 and p2
        p1.set_points(0.0, 0.0);
        p2.set_points(10.0, 10.0);
        line.calc(&p1, &p2);

        assert_eq!(1.0, line.slope.unwrap());
        assert_eq!(0.0, line.intercept.unwrap());
    }

    #[test]
    fn test_line_from() {
        let mut p1 = Point::new();
        let mut p2 = Point::new();
        p1.set_points(0.0, 0.0);
        p2.set_points(10.0, 10.0);

        let line = Line::from((&p1, &p2));
        assert_eq!(1.0, line.slope.unwrap());
        assert_eq!(0.0, line.intercept.unwrap());
        let line2 = Line::from((&p2, &p1));
        assert_eq!(1.0, line2.slope.unwrap());
        assert_eq!(0.0, line2.intercept.unwrap());
    }
}
