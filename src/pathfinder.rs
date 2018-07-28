use std::collections::HashMap;
use std::rc::Rc;

use node::*;

pub struct AStarPathFinder {
    pub field: PathField,
    pub from: Point,
    pub to: Point
}

impl AStarPathFinder {

    pub fn get_path(&self) -> Vec<Point> {
        let initial_node = Node {point : self.from, parent: None, from: &self.from, to: &self.to};

        let mut open   : HashMap<Point,Node> = HashMap::new();
        let mut closed : HashMap<Point,Node> = HashMap::new();

        // was null in the java code
        let mut target_node = initial_node.clone();

        open.insert(self.from, initial_node);

        loop {
            if open.is_empty() {
                return Vec::new();
            }

            let mut min = i32::max_value();
            let mut min_node = None;
        
            for node in open.values() {
                let f = node.f();
                if min_node.is_none() || f < min {
                    min = f;
                    min_node = Some(node.clone());
                }
            }
  
            match min_node {
                Some(m_node) => {
                    if m_node.point.eq(&self.to) {
                        target_node = (m_node).clone();
                        break;
                    }

                    let m_point = m_node.point;

                    let array: [Point; 8] = [
                        Point { x: m_point.x +1, y: m_point.y },
                        Point { x: m_point.x +1, y: m_point.y + 1 },
                        Point { x: m_point.x , y: m_point.y + 1 },
                        Point { x: m_point.x -1, y: m_point.y +1 },
                        Point { x: m_point.x -1, y: m_point.y },
                        Point { x: m_point.x -1, y: m_point.y -1 },
                        Point { x: m_point.x , y: m_point.y -1 },
                        Point { x: m_point.x +1, y: m_point.y -1 }
                    ];

                    for i in 0..7 {
                        let point = array[i];

                        // I do not consider the end point to be occupied, so I can move towards it
                        if self.field.contains(point) && (point.eq(&self.to) || !self.field.occupied_from(point, self.from)) {
                            if !closed.contains_key(&point) {
                                let mut node = Node {point : point.to_owned(), parent: None, from: &self.from, to: &self.to};
                                node.set_parent((m_node).clone());
                                if !open.contains_key(&point) {
                                    open.insert(point, node);
                                } else {
                                    let got = open.get(&point);
                                    let mut got_some = got.unwrap().clone();
                                    let gToMin = m_node.g_of(&got_some);
                                    if gToMin < node.g() {
                                        got_some.set_parent((m_node).clone());
                                    }
                                }
                            }
                        }
                    }
                        
                    closed.insert(m_node.point, (m_node).clone());
                    
                    open.remove(&m_node.point);

                },
                None => {
                    println!("Cannot find min node");
                    break
                }
            }
            
        }

        let mut result : Vec<Point> = Vec::new();

        while target_node.parent.is_some() {
            // the path can contains occupied points. Normally it can be only the end point 
            if !self.field.occupied(target_node.point) {
                result.push(target_node.point);
            }

            target_node = (*target_node.parent.unwrap()).clone();
        }
        return result;
    }

}

#[derive(Clone)]
pub struct Node<'a> {
    point : Point,
    parent: Option<Rc<Node<'a>>>,
    from : &'a Point,
    to : &'a Point
}

impl <'a> Node<'a> {
    pub fn f(&self) -> i32 {
        return self.g() + self.h();
    }
        
    pub fn g(&'a self) -> i32 {
        match self.parent {
            Some(ref node) => {
                return self.g_of(&node);
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
        self.parent = Some(Rc::new(node));
    }

}