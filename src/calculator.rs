use std::collections::HashMap;
pub struct Line {
    slope: Option<f64>,
    intercept: Option<f64>,
}
pub struct Point {
    voltage: Option<f64>,
    physical: Option<f64>,
}

//use this to pass a value into the equation
pub enum MeasurementType {
    voltage(f64),
    physical(f64),
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
    pub fn set_point(&mut self, x: f64, y: f64) {
        self.voltage = Some(x);
        self.physical = Some(y);
    }
    pub fn set_voltage(&mut self, v: f64) {
        self.voltage = Some(v);
    }

    pub fn set_physical(&mut self, p: f64) {
        self.physical = Some(p);
    }

    pub fn get_val(&self) -> HashMap<&str, f64> {
        let mut vals = HashMap::new();
        vals.insert("v", self.voltage.clone().unwrap());
        vals.insert("p", self.physical.clone().unwrap());
        vals
    }
}

impl Line {
    pub fn new() -> Self {
        Line {
            slope: None,
            intercept: None,
        }
    }

    // updates slop and intercept given 2 points
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

    // Get values in uhh hashmap i guess
    pub fn get_val(&self) -> Option<(f64, f64)> {
        if let Some(m) = self.slope {
            if let Some(b) = self.intercept {
                return Some((m, b));
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    // pub fn to take a value of type v or p and
    pub fn get_corresponding_value(&self, value: &MeasurementType) -> Result<(f64), ()> {
        if let (Some(m), Some(b)) = (self.slope.as_ref(), self.intercept.as_ref()) {
            match value {
                MeasurementType::physical(y) => return Ok((y - b) / m),
                MeasurementType::voltage(x) => return Ok(m * x + b),
            }
        }
        Err(())
    }
}

impl From<(&Point, &Point)> for Line {
    fn from(points: (&Point, &Point)) -> Self {
        let mut line = Line::new();
        line.calc(points.0, points.1);
        line
    }
}

impl From<(f64, f64)> for Point {
    fn from(vals: (f64, f64)) -> Self {
        let mut point = Point::new();
        point.set_point(vals.0, vals.1);
        point
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
        p1.set_point(0.0, 0.0);
        p2.set_point(10.0, 10.0);
        line.calc(&p1, &p2);

        assert_eq!(1.0, line.slope.unwrap());
        assert_eq!(0.0, line.intercept.unwrap());
    }

    #[test]
    fn test_line_from() {
        let mut p1 = Point::new();
        let mut p2 = Point::new();
        p1.set_point(0.0, 0.0);
        p2.set_point(10.0, 10.0);

        let line = Line::from((&p1, &p2));
        assert_eq!(1.0, line.slope.unwrap());
        assert_eq!(0.0, line.intercept.unwrap());
        let line2 = Line::from((&p2, &p1));
        assert_eq!(1.0, line2.slope.unwrap());
        assert_eq!(0.0, line2.intercept.unwrap());
    }

    #[test]
    fn create_point_from_trait() {
        let p1 = Point::from((0.0, 0.0));
        let p2 = Point::from((10.0, 10.0));
        let line = Line::from((&p1, &p2));
        assert_eq!(1.0, line.slope.unwrap());
        assert_eq!(0.0, line.intercept.unwrap());
        assert_eq!(
            5.0,
            line.get_corresponding_value(&crate::calculator::MeasurementType::physical(5.0))
                .unwrap()
        );
    }
}
