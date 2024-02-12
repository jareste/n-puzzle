use crate::map::Map;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

enum Path {
    Found(Vec<Map>, usize),
    Minimum(usize),
    Impossible,
}


pub fn a_star(start: &Map, goal: &Map) -> Option<(Vec<Map>, usize)> {

    let mut open_heap = BinaryHeap::new();

    let mut opened = HashSet::new();
    opened.insert(start.clone());
    let mut closed = HashSet::new();

    let mut g_score = HashMap::new();
    g_score.insert(start.clone(), 0);
    
    open_heap.push((start.manhattan_dist(goal), start.clone()));

    while !open_heap.is_empty() {
        let current = open_heap.pop().unwrap();
        let current_node = current.1;
        let current_punt = current.0;

        if current_node == *goal {
            return Some((vec![current_node], current_punt as usize));
        }

        closed.insert(current_node.clone());
        opened.remove(&current_node);

        for (neighbor, cost) in current_node.successors() {
            let in_opened = opened.contains(&neighbor);
            let in_closed = opened.contains(&neighbor);
            let g = g_score.get(&current_node).unwrap() + cost;
            let h = neighbor.manhattan_dist(goal);
            let f = g + h;

            if !in_opened && !in_closed {
                open_heap.push((f, neighbor.clone()));
                g_score.insert(neighbor.clone(), g);
                opened.insert(neighbor.clone());
            } 
            else {
                if *g_score.get(&neighbor).unwrap() > g{
                    g_score.insert(neighbor.clone(), g);
                    if in_opened{
                        open_heap.push((f, neighbor.clone()));
                    }
                    if in_closed{
                        opened.insert(neighbor.clone());
                        closed.remove(&neighbor);
                        open_heap.push((f, neighbor.clone()));
                    }
                }
            }

        }
    }

    return None;
}