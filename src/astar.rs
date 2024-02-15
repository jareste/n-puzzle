use crate::map::Map;
use crate::solver;
use std::collections::BinaryHeap;
use std::collections::HashMap;

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


pub fn a_star(start: &Map, goal: &Map, heuristic: solver::Heuristic, h_method: solver::HMethod, max_p: &usize) -> Option<(Vec<Map>, usize, usize, usize)> {

    let mut open = BinaryHeap::new();

    let mut node_info = HashMap::new();

    open.push((usize::MAX -start.manhattan_dist(goal) as usize, (start.clone(), 0 as usize)));

    let start_f = match heuristic{
        solver::Heuristic::Manhattan => start.manhattan_dist(goal),
        solver::Heuristic::Hamming => start.hamming_dist(goal),
        solver::Heuristic::Euclidean => start.euclidean_dist(goal),
        solver::Heuristic::LinearConflicts => start.manhattan_linear_conflicts(goal),
    };

    node_info.insert(start.clone(), Node {
        g: 0,
        f: start_f as usize,
        parent: None,
    });

    let mut time_c = 1;
    

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
            return Some((path, current_g as usize, time_c, time_c));
        }

        if current_g > node_info[&current_node].g || current_g >= *max_p {
            continue;
        }

        for (neighbor, cost) in current_node.successors() {
            let new_g = current_g + cost as usize;

            let new_h = match heuristic{
                solver::Heuristic::Manhattan => neighbor.manhattan_dist(goal),
                solver::Heuristic::Hamming => neighbor.hamming_dist(goal),
                solver::Heuristic::Euclidean => neighbor.euclidean_dist(goal),
                solver::Heuristic::LinearConflicts => neighbor.manhattan_linear_conflicts(goal),
            };

            let new_f = match h_method{
                solver::HMethod::Normal => new_g + new_h as usize,
                solver::HMethod::Greedy => new_h as usize,
                solver::HMethod::Uniform => new_g,
            };

            if node_info.contains_key(&neighbor) {
                if new_f < node_info[&neighbor].f {
                    node_info.insert(neighbor.clone(), Node {
                        g: new_g,
                        f: new_f,
                        parent: Some(current_node.clone()),
                    });
                    open.push((usize::MAX -new_f, (neighbor.clone(), new_g)));
                    time_c += 1;
                }
            } else {
                node_info.insert(neighbor.clone(), Node {
                    g: new_g,
                    f: new_f,
                    parent: Some(current_node.clone()),
                });
                open.push((usize::MAX -new_f, (neighbor.clone(), new_g)));
                time_c += 1;
            }

        }
    }

    return None;
}
