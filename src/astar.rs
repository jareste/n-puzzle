// use crate::map::Map;
// use std::collections::BinaryHeap;
// use std::collections::HashMap;

// enum Path {
//     Found(Vec<Map>, usize),
//     Minimum(usize),
//     Impossible,
// }

// fn reconstruct_path(path: HashMap<Map, Map>,goal: &Map) -> Vec<Map> {
//     let mut current = Vec::new();
//     current.push(goal);
//     while let Some(parent) = path.get(current.last().unwrap()){
//         current.push(parent);
//     }
//     current.reverse();
//     return current;
// }

// pub fn a_star(start: &Map, goal: &Map) -> Option<(Vec<Map>, usize)> {

//     let mut open = BinaryHeap::new();

//     let mut came_from = HashMap::new();
    
//     let mut g_score = HashMap::new();
//     g_score.insert(start, 0);

//     open.push((g_score[start], start));

//     while !open.is_empty() {
//         let current = open.pop().unwrap().1;

//         if current == goal {
//             return Some((reconstruct_path(came_from, goal), g_score[&current]));
//         }

//         for (neighbor, cost) in current.successors() {
//             let tentative_g_score = g_score[&current] + cost as usize;
            
//             if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&usize::MAX) {
//                 came_from.insert(&neighbor, current);
//                 g_score.insert(&neighbor, tentative_g_score);
//                 if !open.iter().any(|(_, n)| n == &&neighbor) {
//                     open.push((tentative_g_score + neighbor.manhattan_dist(goal) as usize, &neighbor));
//                 }
//             }
//         }
//     }
//     return None;
// }