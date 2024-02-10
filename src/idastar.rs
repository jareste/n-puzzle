use crate::map::Map;


enum Path {
    Found(Vec<Map>, usize),
    Minimum(usize),
    Impossible,
}

pub fn ida_star(start: &Map, goal: &Map) -> Option<(Vec<Map>, usize)> {

    let mut bound = start.manhattan_dist(goal) as usize;
    let mut path = vec![start.clone()];

    loop {
        match search(&mut path, 0, bound, goal) {
            Path::Found(p, c) => return Some((p, c)),
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

fn search(path: & mut Vec<Map>, g: usize, bound: usize, goal: &Map) -> Path {
    let node = path.last().unwrap();

    let f = g + node.manhattan_dist(goal) as usize;

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
            match search(path, g + cost as usize, bound, goal) {
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