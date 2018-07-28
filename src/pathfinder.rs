use std::collections::HashMap;
use std::borrow::Borrow;

use node::*;

pub struct AStarPathFinder {
    pub field: PathField,
    pub from: Point,
    pub to: Point
}

impl AStarPathFinder {

    pub fn get_path(&self) -> Vec<Point> {
        let initial_node = Node {point : self.from, parent: None, from: &self.from, to: &self.to};

        let mut mNode : Node;
        let mut minNode : Option<Node>;

        let mut open   : HashMap<Point,Node> = HashMap::new();
        let mut closed : HashMap<Point,Node> = HashMap::new();

        // was null in the java code
        let mut targetNode = initial_node.to_owned();
        open.insert(self.from, initial_node.to_owned());

        while true {
            if open.is_empty() {
                return Vec::new();
            }

            let mut min = i32::max_value();
            minNode = None;
        
            for node in open.values() {
                let f = node.f();
                if minNode.is_none() || f < min {
                    min = f;
                    minNode = Some(*node);
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

            mNode = minNode.unwrap();
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

            for i in 0..7 {
                let point = array[i];

                // I do not consider the end point to be occupied, so I can move towards it
                if self.field.contains(point) && (point.eq(&self.to) || !self.field.occupied_from(point, self.from)) {
                    if !closed.contains_key(&point) {
                        let mut node = Node {point : point.to_owned(), parent: None, from: &self.from, to: &self.to};
                        node.set_parent(mNode);
                        if !open.contains_key(&point) {
                            open.insert(point, node.to_owned());
                        } else {
                            let got = open.get(&point);
                            let mut got_some = *got.unwrap();
                            let gToMin = mNode.g_of(&got_some);
                            if gToMin < node.g() {
                                got_some.set_parent(mNode);
                            }
                        }
                    }
                }
            }
                
            closed.insert(mNode.point, mNode);
            
            open.remove(mNode.point.borrow());
            
        }

        let mut result : Vec<Point> = Vec::new();

        while targetNode.parent.is_some() {
            // the path can contains occupied points. Normally it can be only the end point 
            if !self.field.occupied(targetNode.point) {
                result.push(targetNode.point);
            }

            targetNode = *targetNode.parent.unwrap();
        }
        return result;
    }

}

#[derive(Copy, Clone)]
pub struct Node<'a> {
    point : Point,
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

    pub fn set_parent(&mut self, node: Node<'a>) {
        self.parent = Some(&node.to_owned());
    }

}