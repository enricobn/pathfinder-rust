use std::collections::HashMap;

use node::*;

pub struct AStarPathFinder {
    pub field: PathField,
    pub from: Point,
    pub to: Point
}

impl AStarPathFinder {

    pub fn get_path(&self) -> Vec<Point> {
        let mut open : HashMap<Point,Node> = HashMap::new();
        let mut closed : HashMap<Point,Node> = HashMap::new();

        // was null in the java code
        let mut targetNode = Node {point : &self.from, parent: None, from: &self.from, to: &self.to};

        open.insert(self.from, targetNode);

        while true {
//            out.print(_open.size() + " ");
            
            if open.is_empty() {
                return Vec::new();
            }

                let mut min = i32::max_value();
                let mut minNode : Option<Node> = None;
            
//                    Node orderderMinNode = _open.iterator().next();
                    for node in open.values() {
                        let f = node.f();
                        if minNode.is_none() || f < min {
                            min = f;
                            minNode = Some(*node);
                        }
                    }

//                  synchronized (_open) {
//                      for (Object o : _open) {
//                          System.out.println("point=" + o);
//                      }
//                  }
            
//                Node minNode = (Node) _open.remove();

//                out.println(minNode);
                
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

                for point in array.iter() {
                    // I do not consider the end point to be occupied, so I can move towards it
                    if self.field.contains(*point) && (point.eq(&self.to) || !self.field.occupied_from(*point, self.from)) {
                        if !closed.contains_key(point) {
                            let node = Node {point : point, parent: Some(Box::new(mNode)), from: &self.from, to: &self.to};
                            let got = open.get(point);
                            if got.is_none() {
                                open.insert(*point, node);
                            } else {
                                let gToMin = mNode.g_of(*got.unwrap());
                                if gToMin < node.g() {
                                    got.unwrap().setParent(minNode);
                                }
                            }
                        }
                    }
                }
                
                closed.insert(*mNode.point, mNode);
                
                open.remove(mNode.point);
                
//                System.out.println(_open.size());
        }

        let result : Vec<Point> = Vec::new();

        while targetNode.parent.is_some() {
            // the path can contains occupied points. Normally it can be only the end point 
            if !self.field.occupied(*targetNode.point) {
                result.push(*targetNode.point);
            }
            targetNode = *targetNode.parent.unwrap();
        }
        return result;
    }

}

pub struct Node<'a> {
    point : &'a Point,
    parent: Option<Box<Node<'a>>>,
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
                return self.g_of(*node);
            },
            None => 0
        }
    }

    pub fn g_of(&self, node: Node) -> i32 {
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

}