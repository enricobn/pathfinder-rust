use std::collections::HashMap;

use node::*;

pub struct AStarPathFinder {
    pub field: PathField,
    pub from: Point,
    pub to: Point
}

impl AStarPathFinder {

    pub fn get_path() -> Vec<Point> {
        let mut open : HashMap<Point,Node> = HashMap::new();
        let mut closed : HashMap<Point,Node> = HashMap::new();

        return Vec::new();
    }

}

pub struct Node {
    point : Point,
    parent: Box<Node>,
    from : Point,
    to : Point
}

impl Node {
    pub fn f(&self) -> i32 {
        return self.g() + self.h();
    }
        
    pub fn g(&self) -> i32{
        return self.g_(&self.parent);
    }

    pub fn h(&self) -> i32 {
        return ((self.to.x - self.point.x).abs() + (self.to.y - self.point.y).abs()) * 10;
    }

    fn g_(&self, node : &Node) -> i32 {
        let mut g = node.g();
        if self.point.x == node.point.x || self.point.y == node.point.y {
            g += 10;
        } else {
            g += 14;
        }
        return g;
    }
}