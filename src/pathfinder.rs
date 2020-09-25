use std::collections::HashMap;
use std::rc::Rc;

use base::*;

pub struct AStarPathFinder {
    pub field: PathField,
    pub from: Point,
    pub to: Point
}

impl AStarPathFinder {

    pub fn get_path(&self) -> Vec<Point> {
        let initial_node = Node::new(self.from, None, &self.from, &self.to);

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
                let f = node.f;
                if min_node.is_none() || f < min {
                    min = f;
                    min_node = Some(node.clone());
                }
            }
  
            match min_node {
                Some(m_node) => {
                    if m_node.point.eq(&self.to) {
                        target_node = m_node.clone();
                        break;
                    }

                    let m_point = m_node.point;

                    let array: [Point; 8] = [
                        Point { x: m_point.x + 1, y: m_point.y },
                        Point { x: m_point.x + 1, y: m_point.y + 1 },
                        Point { x: m_point.x , y: m_point.y + 1 },
                        Point { x: m_point.x - 1, y: m_point.y + 1 },
                        Point { x: m_point.x - 1, y: m_point.y },
                        Point { x: m_point.x - 1, y: m_point.y - 1 },
                        Point { x: m_point.x , y: m_point.y - 1 },
                        Point { x: m_point.x + 1, y: m_point.y - 1 }
                    ];

                    for i in 0..7 {
                        let point = array[i];

                        // I do not consider the end point to be occupied, so I can move towards.
                        if self.field.contains(point) && (point.eq(&self.to) || !self.field.occupied(point)) {
                            if !closed.contains_key(&point) {
                                let node = Node::new(point.to_owned(), Some(m_node.clone()), &self.from, &self.to);
                                if !open.contains_key(&point) {
                                    open.insert(point, node);
                                } else {
                                    let got = open.get_mut(&point).unwrap();
                                    let g_to_min = m_node.g_of(&got);
                                    if g_to_min < node.g {
                                        got.set_parent(m_node.clone());
                                    }
                                }
                            }
                        }
                    }
                        
                    open.remove(&m_node.point);

                    closed.insert(m_node.point, m_node);

                },
                None => {
                    println!("Cannot find min node");
                    break
                }
            }
            
        }

        let mut result : Vec<Point> = Vec::new();

        while target_node.parent.is_some() {
            // The path can contains occupied points. Normally it can be only the end point.
            if !self.field.occupied(target_node.point) {
                result.push(target_node.point);
            }

            target_node = (*target_node.parent.unwrap()).clone();
        }
        return result;
    }

}

#[derive(Clone)]
struct Node<'a> {
    point : Point,
    parent: Option<Rc<Node<'a>>>,
    from : &'a Point,
    to : &'a Point,
    g: i32,
    f: i32,
    h: i32
}

impl <'a> Node<'a> {

    pub fn new(point: Point, parent: Option<Node<'a>>, from : &'a Point, to : &'a Point) -> Node<'a> {
        let mut node = Node { point, parent: None, from, to, g: 0, f: 0,
            h: ((to.x - point.x).abs() + (to.y - point.y).abs()) * 10 };
        if parent.is_some() {
            node.set_parent(parent.unwrap());
        } else {
            node.f = node.g + node.h;
        }
        return node;
    }

    pub fn g_of(&self, node: &'a Node<'a>) -> i32 {
        let mut g = node.g;
        if self.point.x == node.point.x || self.point.y == node.point.y {
            g += 10;
        } else {
            g += 14;
        }
        return g;
    }

    pub fn set_parent(&mut self, node: Node<'a>) {
        self.g = self.g_of(&node);
        self.f = self.g + self.h;
        self.parent = Some(Rc::new(node));
    }

}