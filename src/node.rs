use std::fmt;

pub struct Point {pub x: i32, pub y:i32}

impl Point {
    fn mv(&self, x_diff: i32, y_diff: i32) -> Point {
        return Point {x: self.x + x_diff, y: self.y + y_diff};
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

pub trait FieldShape {

    fn location(&self) -> Point;
    
    fn set_location(&self, location: Point) -> FieldShape;
    
    fn contains(&self, point: Point) -> bool;
    
    fn mv(&self, x_diff: i32, y_diff: i32) -> FieldShape;
    
    fn is_moving(&self) -> bool;
        
}

pub struct PathField<'a> {
    shapes: Vec<&'a FieldShape>,
    size: Dimension,
    rectangle: Rectangle
}

impl <'a> PathField<'a> {
    fn add<T: Sized + FieldShape>(&mut self, field_shape: &'a T) {
        self.shapes.push(field_shape);
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