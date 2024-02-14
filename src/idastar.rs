use crate::{map::Map, solver};


enum Path {
    Found(Vec<Map>, usize),
    Minimum(usize),
    Impossible,
}

pub fn ida_star(start: &Map, goal: &Map, heuristic: solver::Heuristic, h_method: solver::HMethod) -> Option<(Vec<Map>, usize, usize, usize)> {

    let mut bound = start.manhattan_dist(goal) as usize;
    let mut path = vec![start.clone()];

    loop {
        match search(&mut path, 0, bound, goal, &heuristic, &h_method) {
            Path::Found(p, c, time_c, space_c) => return Some((p, c, time_c, space_c)),
            Path::Minimum(c) => {
                if bound == c{
                    return None;
                }
                bound = c;
            }
            Path::Impossible => return None,
        }
    }

    
}

fn search(path: & mut Vec<Map>, g: usize, bound: usize, goal: &Map, heuristic: &solver::Heuristic, h_method: &solver::HMethod) -> Path {
    let node = path.last().unwrap();

    let h = match heuristic{
        solver::Heuristic::Manhattan => node.manhattan_dist(goal),
        solver::Heuristic::Hamming => node.hamming_dist(goal),
        solver::Heuristic::Euclidean => node.euclidean_dist(goal),
        solver::Heuristic::LinearConflicts => node.manhattan_linear_conflicts(goal),
    };

    let f = match h_method{
        solver::HMethod::Normal => g + h as usize,
        solver::HMethod::Greedy => h as usize,
        solver::HMethod::Uniform => g,
    };

    if f > bound {
        return Path::Minimum(f);
    }
    if node == goal {
        return Path::Found(path.clone(), g);
    }
    let mut min = usize::MAX;

    for (succ, cost) in node.successors() {
        if !path.contains(&succ) {
            path.push(succ);
            match search(path, g + cost as usize, bound, goal, heuristic, h_method) {
                Path::Found(p, c) => return Path::Found(p, c),
                Path::Minimum(c) => min = min.min(c),
                Path::Impossible => return Path::Impossible,
            }
            path.pop();
        }
    }
    if min == usize::MAX {
        return Path::Impossible;
    }
    Path::Minimum(min)
}