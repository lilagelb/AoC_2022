use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
use std::rc::Rc;
use crate::day::{Answer, Day};

pub struct Day12;
impl Day for Day12 {
    type TypePart1 = u32;
    type TypePart2 = u32;

    fn run(&mut self) -> Answer<Self::TypePart1, Self::TypePart2> {
        let mut input: Vec<Vec<u8>> = self.get_input_for_day_by_line(12).iter()
            .map(|row| row.as_bytes().to_vec())
            .collect();

        let mut graph = Graph::new();
        let mut head = (0usize, 0usize);
        let mut tail = (0usize, 0usize);
        let mut elevation_a_options = HashMap::new();

        for y in 0..input.len() {
            for x in 0..input[0].len() {
                if input[y][x] == 'S' as u8 {
                    head = (x, y);
                    input[y][x] = 'a' as u8;
                } else if input[y][x] == 'E' as u8 {
                    tail = (x, y);
                    input[y][x] = 'z' as u8;
                } else if input[y][x] == 'a' as u8 {
                    elevation_a_options.insert((x, y), RefCell::new(u32::MAX));
                }
            }
        }
        for y in 0..input.len() {
            for x in 0..input[0].len() {
                let mut links: Vec<(usize, usize)> = Vec::new();
                for direction in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let neighbour_x = x as i32 + direction.0;
                    let neighbour_y = y as i32 + direction.1;
                    if 0 <= neighbour_x && neighbour_x < input[0].len() as i32
                        && 0 <= neighbour_y && neighbour_y < input.len() as i32
                    {
                        if input[y][x] + 1 >= input[neighbour_y as usize][neighbour_x as usize] {
                            links.push((neighbour_x as usize, neighbour_y as usize));
                        }
                    }
                }
                graph.add_node((x, y), links);

            }
        }


        for start_option in elevation_a_options.keys() {
            graph.reset_for_search();
            *elevation_a_options[start_option].borrow_mut()
                = graph.breadth_first_search(*start_option, tail);
        }

        let part_2 = elevation_a_options.values()
            .map(|elem| *elem.borrow())
            .min()
            .unwrap();

        graph.reset_for_search();
        let part_1 = graph.breadth_first_search(head, tail);

        Answer::new(Some(part_1), Some(part_2))
    }
}
impl Day12 {
    pub fn new() -> Day12 {
        Day12
    }
}

struct Graph<T: Hash + Eq + Copy> {
    nodes: HashMap<T, NodePtr<T>>,
}
impl<T: Hash + Eq + Copy> Graph<T> {
    fn new() -> Graph<T> {
        Graph {
            nodes: HashMap::new(),
        }
    }

    fn add_node(&mut self, identifier: T, links: Vec<T>) {
        if !self.nodes.contains_key(&identifier) {
            self.nodes.insert(identifier, NodePtr::new(Node::new(identifier)));
        }

        for link in links {
            if !self.nodes.contains_key(&link) {
                self.nodes.insert(link, NodePtr::new(Node::new(link)));
            }
            self.nodes[&identifier].borrow_mut()
                .arcs.push(NodePtr::clone(&self.nodes[&link]));
        }
    }

    fn reset_for_search(&mut self) {
        for node in self.nodes.values() {
            node.borrow_mut().reset_for_search();
        }
    }

    fn breadth_first_search(&self, from: T, to: T) -> u32 {
        let mut node_queue = Vec::new();
        let mut current_node = NodePtr::clone(&self.nodes[&from]);
        current_node.borrow_mut().distance = 0;

        loop {
            for arc in &current_node.borrow().arcs {
                if arc.borrow().added_to_queue {
                    continue;
                }
                let proposed_distance = current_node.borrow().distance + 1;
                if arc.borrow().distance > proposed_distance {
                    arc.borrow_mut().distance = proposed_distance;
                }

                node_queue.push(NodePtr::clone(arc));
                arc.borrow_mut().added_to_queue = true;
            }
            if current_node.borrow().identifier == to {
                break current_node.borrow().distance;
            }
            if node_queue.len() == 0 {
                break u32::MAX;
            }
            current_node = node_queue.remove(0);
        }
    }
}

struct Node<T> {
    identifier: T,
    arcs: Vec<NodePtr<T>>,
    distance: u32,
    added_to_queue: bool,
}
impl<T> Node<T> {
    fn new(identifier: T) -> Node<T> {
        Node {
            identifier,
            arcs: Vec::new(),
            distance: u32::MAX,
            added_to_queue: false,
        }
    }
    fn reset_for_search(&mut self) {
        self.distance = u32::MAX;
        self.added_to_queue = false;
    }
}

struct NodePtr<T>(Rc<RefCell<Node<T>>>);
impl<T> Deref for NodePtr<T> {
    type Target = Rc<RefCell<Node<T>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> Clone for NodePtr<T> {
    fn clone(&self) -> Self {
        NodePtr(Rc::clone(&self.0))
    }
}
impl<T> NodePtr<T> {
    fn new(node: Node<T>) -> NodePtr<T> {
        NodePtr(Rc::new(RefCell::new(node)))
    }
}