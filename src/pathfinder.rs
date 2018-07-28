use std::collections::HashMap;

use node::*;

pub struct AStarPathFinder<'a> {
    pub field: PathField,
    pub from: &'a Point,
    pub to: &'a Point
}

impl <'a> AStarPathFinder<'a> {

    pub fn get_path(&'a self) -> Vec<Point> {
        // was null in the java code

        let initial_node = Node {point : &self.from, parent: None, from: &self.from, to: &self.to};
        let mut targetNode = &initial_node;

        let mut open   : HashMap<&'a Point,&Node<'a>> = HashMap::new();
        let mut closed : HashMap<&'a Point,&Node<'a>> = HashMap::new();

        open.insert(self.from, &initial_node);

        while true {
            if open.is_empty() {
                return Vec::new();
            }

            let mut min = i32::max_value();
            let mut minNode : Option<&Node<'a>> = None;
        
            for node in open.values() {
                let f = node.f();
                if minNode.is_none() || f < min {
                    min = f;
                    minNode = Some(node);
                }
            }
  
            match minNode {
                Some(mNode) => {
                    if mNode.point.eq(&self.to) {
                        targetNode = mNode;
                        break;
                    }
                },
                None => {
                    println!("Cannot find min node");
                    break
                }
            }

            let mNode = minNode.unwrap();
            let mPoint = mNode.point;

            let array: [Point; 8] = [
                Point { x: mPoint.x +1, y: mPoint.y },
                Point { x: mPoint.x +1, y: mPoint.y + 1 },
                Point { x: mPoint.x , y: mPoint.y + 1 },
                Point { x: mPoint.x -1, y: mPoint.y +1 },
                Point { x: mPoint.x -1, y: mPoint.y },
                Point { x: mPoint.x -1, y: mPoint.y -1 },
                Point { x: mPoint.x , y: mPoint.y -1 },
                Point { x: mPoint.x +1, y: mPoint.y -1 }
            ];

/*
            for point in array.iter() {
                // I do not consider the end point to be occupied, so I can move towards it
                if self.field.contains(*point) && (point.eq(&self.to) || !self.field.occupied_from(*point, *self.from)) {
                    if !closed.contains_key(point) {
                        let node = Node {point : point, parent: Some(mNode), from: &self.from, to: &self.to};
                        let mut got = open.get(point);
                        if got.is_none() {
                            open.insert(point, &node);
                        } else {
                            let mut got_some = *got.unwrap();
                            let gToMin = mNode.g_of(got_some);
                            if gToMin < node.g() {
                                got_some.set_parent(mNode);
                            }
                        }
                    }
                }
            }
            */
                
            closed.insert(mNode.point, mNode);
            
            open.remove(mNode.point);
            
        }

        let mut result : Vec<Point> = Vec::new();

        while targetNode.parent.is_some() {
            // the path can contains occupied points. Normally it can be only the end point 
            if !self.field.occupied(*targetNode.point) {
                result.push(*targetNode.point);
            }

            targetNode = targetNode.parent.unwrap();
        }
        return Vec::new();
    }

}

pub struct Node<'a> {
    point : &'a Point,
    parent: Option<&'a Node<'a>>,
    from : &'a Point,
    to : &'a Point
}

impl <'a> Node<'a> {
    pub fn f(&self) -> i32 {
        return self.g() + self.h();
    }
        
    pub fn g(&self) -> i32 {
        match self.parent {
            Some(node) => {
                return self.g_of(node);
            },
            None => 0
        }
    }

    pub fn g_of(&self, node: &'a Node<'a>) -> i32 {
        let mut g = node.g();
        if self.point.x == node.point.x || self.point.y == node.point.y {
            g += 10;
        } else {
            g += 14;
        }
        return g;        
    }

    pub fn h(&self) -> i32 {
        return ((self.to.x - self.point.x).abs() + (self.to.y - self.point.y).abs()) * 10;
    }

    pub fn set_parent(&mut self, node: &'a Node<'a>) {
        self.parent = Some(node);
    }

}