use std::{
    cmp::{max, min},
    num::ParseIntError,
    str::FromStr,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    x: u32,
    y: u32,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap_or_default();
        let x = x.parse()?;
        let y = y.parse()?;

        Ok(Point { x, y })
    }
}

#[derive(Debug, PartialEq)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    pub fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    pub fn points(&self) -> Vec<Point> {
        let mut points = Vec::new();

        if self.is_vertical() {
            let start_y = min(self.start.y, self.end.y);
            let end_y = max(self.start.y, self.end.y);
            for i in start_y..=end_y {
                points.push(Point {
                    x: self.start.x,
                    y: i,
                });
            }
        } else if self.is_horizontal() {
            let start_x = min(self.start.x, self.end.x);
            let end_x = max(self.start.x, self.end.x);
            for i in start_x..=end_x {
                points.push(Point {
                    x: i,
                    y: self.start.y,
                });
            }
        } else {
            points.push(self.start);

            let mut mid_point = self.start;
            while mid_point != self.end {
                if mid_point.x < self.end.x {
                    mid_point.x += 1;
                } else if mid_point.x > self.end.x {
                    mid_point.x -= 1;
                }

                if mid_point.y < self.end.y {
                    mid_point.y += 1;
                } else if mid_point.y > self.end.y {
                    mid_point.y -= 1;
                }

                points.push(mid_point);
            }
        }

        points
    }
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once(" -> ").unwrap_or_default();

        let line = Line {
            start: start.parse()?,
            end: end.parse()?,
        };

        Ok(line)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_from_str() {
        let point = "0,9";
        let expected = Point { x: 0, y: 9 };

        assert_eq!(Point::from_str(point).unwrap(), expected);
    }

    #[test]
    fn line_from_str() {
        let line = "0,9 -> 5,9";
        let expected = Line {
            start: Point { x: 0, y: 9 },
            end: Point { x: 5, y: 9 },
        };

        assert_eq!(Line::from_str(line).unwrap(), expected);
    }

    #[test]
    fn test_line_is_horizontal() {
        let line = Line::from_str("0,9 -> 5,9").unwrap();
        assert!(line.is_horizontal());

        let line = Line::from_str("7,0 -> 7,4").unwrap();
        assert!(!line.is_horizontal());
    }

    #[test]
    fn test_line_is_vertical() {
        let line = Line::from_str("7,0 -> 7,4").unwrap();
        assert!(line.is_vertical());

        let line = Line::from_str("0,9 -> 5,9").unwrap();
        assert!(!line.is_vertical());
    }

    #[test]
    fn test_line_points() {
        let line = Line::from_str("7,0 -> 7,4").unwrap();
        let expected = vec![
            Point::from_str("7,0").unwrap(),
            Point::from_str("7,1").unwrap(),
            Point::from_str("7,2").unwrap(),
            Point::from_str("7,3").unwrap(),
            Point::from_str("7,4").unwrap(),
        ];

        assert_eq!(line.points(), expected);

        let line = Line::from_str("3,4 -> 1,4").unwrap();
        let expected = vec![
            Point::from_str("1,4").unwrap(),
            Point::from_str("2,4").unwrap(),
            Point::from_str("3,4").unwrap(),
        ];

        assert_eq!(line.points(), expected);

        let line = Line::from_str("1,1 -> 3,3").unwrap();
        let expected = vec![
            Point::from_str("1,1").unwrap(),
            Point::from_str("2,2").unwrap(),
            Point::from_str("3,3").unwrap(),
        ];

        assert_eq!(line.points(), expected);

        let line = Line::from_str("9,7 -> 7,9").unwrap();
        let expected = vec![
            Point::from_str("9,7").unwrap(),
            Point::from_str("8,8").unwrap(),
            Point::from_str("7,9").unwrap(),
        ];

        assert_eq!(line.points(), expected);
    }
}
