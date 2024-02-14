
use core::panic;

use crate::map::Map;
use crate::idastar::ida_star;
use crate::astar::a_star;
use pathfinding::prelude::idastar;

pub enum Heuristic {
    Manhattan,
    Hamming,
    Euclidean,
    LinearConflicts,
}

pub enum HMethod {
    Normal,
    Greedy,
    Uniform,
}

fn str_to_heuristic(heuristic: &str) -> Heuristic {
    match heuristic {
        "manhattan" => Heuristic::Manhattan,
        "hamming" => Heuristic::Hamming,
        "euclidean" => Heuristic::Euclidean,
        "linear_conflicts" => Heuristic::LinearConflicts,
        _ => panic!("Invalid heuristic"),
    }
}

fn str_to_method(h_method: &str) -> HMethod {
    match h_method {
        "normal" => HMethod::Normal,
        "greedy" => HMethod::Greedy,
        "uniform" => HMethod::Uniform,
        _ => panic!("Invalid method"),
    }
}


pub fn solver(heuristic: &str,algorithm: &str, h_method: &str,start: &Map, goal: &Map){
    if algorithm == "default"{
        let result = idastar(start, |m| m.successors(), |m| m.manhattan_dist(goal),|m| *m == *goal);
        println!("Result: {:?}", result);
    }
    else{
        let result = match algorithm {
            "ida_star" => ida_star(&start, &goal, str_to_heuristic(heuristic), str_to_method(h_method)),
            "a_star" => a_star(&start, &goal, str_to_heuristic(heuristic), str_to_method(h_method)),
            _ => panic!("Invalid algorithm"),
        };
        println!("Result: {:?}", result);
    }
   

}
