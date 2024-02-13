use crate::map::Map;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

enum Path {
    Found(Vec<Map>, usize),
    Minimum(usize),
    Impossible,
}

struct Node {
    g: usize,
    f: usize,
    parent: Option<Map>,
}


pub fn a_star(start: &Map, goal: &Map) -> Option<(Vec<Map>, usize)> {

    let mut open = BinaryHeap::new();

    let mut node_info = HashMap::new();

    open.push((usize::MAX -start.manhattan_dist(goal) as usize, (start.clone(), 0 as usize)));

    node_info.insert(start.clone(), Node {
        g: 0,
        f: start.manhattan_dist(goal) as usize,
        parent: None,
    });

    

    while !open.is_empty() {
        let current = open.pop().unwrap();
        let current_node = current.1.0;
        //let current_h = current.0;
        let current_g = current.1.1;

        if current_node == *goal {
            let mut path = vec![];
            let mut current = current_node;
            while current != *start {
                path.push(current.clone());
                current = node_info[&current].parent.clone().unwrap();
            }
            path.reverse();
            return Some((path, current_g as usize));
        }

        if current_g > node_info[&current_node].g {
            continue;
        }

        for (neighbor, cost) in current_node.successors() {
            let new_g = current_g + cost as usize;
            let new_h = neighbor.manhattan_dist(goal);
            let new_f = new_g + new_h as usize;

            if node_info.contains_key(&neighbor) {
                if new_f < node_info[&neighbor].f {
                    node_info.insert(neighbor.clone(), Node {
                        g: new_g,
                        f: new_f,
                        parent: Some(current_node.clone()),
                    });
                    open.push((usize::MAX -new_f, (neighbor.clone(), new_g)));
                }
            } else {
                node_info.insert(neighbor.clone(), Node {
                    g: new_g,
                    f: new_f,
                    parent: Some(current_node.clone()),
                });
                open.push((usize::MAX -new_f, (neighbor.clone(), new_g)));
            }

        }
    }

    return None;
}