use std::fmt;
use std::num;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Point {pub x: i32, pub y:i32}

impl Point {
    fn mv(&self, x_diff: i32, y_diff: i32) -> Point {
        return Point {x: self.x + x_diff, y: self.y + y_diff};
    }

    fn distance(&self, point: Point) -> i32 {
        return (((self.x - point.x) ^ 2 + (self.y - point.y) ^ 2) as f64).sqrt() as i32;
    }

    fn angle(&self, point: Point) -> f32 {
        let dx = point.x - self.x;
        let dy = point.y - self.y;
        // - since 0,0 is up left
        return (dy as f32).atan2(dx as f32);
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

pub trait FieldShape {

    fn location(&self) -> Point;
    
    fn set_location(&self, location: Point) -> Self where Self: Sized;
    
    fn contains(&self, point: Point) -> bool;
    
    fn mv(&self, x_diff: i32, y_diff: i32) -> Self where Self: Sized;
    
    fn is_moving(&self) -> bool;
        
}

pub struct PointFieldShape {
    pub x : i32,
    pub y : i32,
    pub moving : bool
}

impl FieldShape for PointFieldShape {

    fn location(&self) -> Point {
        return Point {x: self.x, y: self.y};
    }
    
    fn set_location(&self, location: Point) -> Self where Self: Sized {
        return PointFieldShape {x: location.x, y: location.y, moving: self.moving};
    }
    
    fn contains(&self, point: Point) -> bool {
        return point.x == self.x && point.y == self.y;
    }
    
    fn mv(&self, x_diff: i32, y_diff: i32) -> Self where Self: Sized {
        return PointFieldShape {x: self.location().x + x_diff, y: self.location().y + y_diff, moving: self.moving};
    }
    
    fn is_moving(&self) -> bool {
        return self.moving;
    }

}

pub struct PathField {
    shapes: Vec<Box<FieldShape>>,
    size: Dimension,
    rectangle: Rectangle
}

impl PathField {

    pub fn occupied(&self, point: Point, from: Point) -> bool {
        let near = point.distance(from) < 3;
        for fieldShape in self.shapes.iter() {
            if fieldShape.contains(point) {
                if !fieldShape.is_moving() || near {
                    return true;
                }
            }
        }
        return false;
    }

}

pub struct Rectangle {
    point: Point,
    width: i32,
    height: i32
}

impl Rectangle {
    pub fn mv(&self, x_diff: i32, y_diff: i32) -> Rectangle {
        return Rectangle {point: self.point.mv(x_diff, y_diff), width: self.width, height: self.height};
    }
    
    pub fn contains(&self, point: Point) -> bool {
        return point.x >= self.point.x && point.x <= self.max_x()
            && point.y >= self.point.y && point.y <= self.max_y();
    }

    pub fn max_x(&self) -> i32 {
        return self.point.x + self.width -1;
    }

    pub fn max_y(&self) -> i32 {
        return self.point.y + self.height -1;
    }
}

pub struct Dimension {
    width: i32,
    height: i32
}