use crate::{map::Map, solver};


#[allow(dead_code)]
enum Path {
    Found(Vec<Map>, usize),
    Minimum(usize),
    Impossible,
}

pub fn ida_star(start: &Map, goal: &Map, heuristic: solver::Heuristic, h_method: solver::HMethod, max_p: &usize) -> Option<(Vec<Map>, usize, usize, usize)> {

    let mut bound =match heuristic{
        solver::Heuristic::Manhattan => start.manhattan_dist(goal) as usize,
        solver::Heuristic::Hamming => start.hamming_dist(goal) as usize,
        solver::Heuristic::Euclidean => start.euclidean_dist(goal) as usize,
        solver::Heuristic::LinearConflicts => start.manhattan_linear_conflicts(goal) as usize,
        solver::Heuristic::NoAdmisible => (start.manhattan_dist(goal) * start.manhattan_dist(goal)) as usize,
    };
    let mut path = vec![start.clone()];

    let mut time_c: usize = 0;
    let mut space_c: usize = 0;

    loop {
        match search(&mut path, 0, bound, goal, &heuristic, &h_method, &mut time_c, &mut space_c) {
            Path::Found(p, c) => return Some((p, c, time_c, space_c)),
            Path::Minimum(c) => {
                if bound == c{
                    return None;
                }
                if c >= *max_p{
                    return None;
                }
                bound = c;
            }
            Path::Impossible => return None,
        }
    }

    
}

fn search(path: & mut Vec<Map>, g: usize, bound: usize, goal: &Map, heuristic: &solver::Heuristic, h_method: &solver::HMethod, time_c: &mut usize, space_c: &mut usize) -> Path {
    let node = path.last().unwrap();
    *time_c += 1;
    if (path.len() as usize) > *space_c {
        *space_c = path.len();
    }

    let h = match heuristic{
        solver::Heuristic::Manhattan => node.manhattan_dist(goal),
        solver::Heuristic::Hamming => node.hamming_dist(goal),
        solver::Heuristic::Euclidean => node.euclidean_dist(goal),
        solver::Heuristic::LinearConflicts => node.manhattan_linear_conflicts(goal),
        solver::Heuristic::NoAdmisible => node.manhattan_dist(goal) * node.manhattan_dist(goal),
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
            match search(path, g + cost as usize, bound, goal, heuristic, h_method, time_c, space_c) {
                Path::Found(p, c) => return Path::Found(p, c),
                Path::Minimum(c) => min = min.min(c),
                Path::Impossible => return Path::Impossible,
            }
            path.pop();
        }
    }
    Path::Minimum(min)
}