
use core::panic;

use crate::map::Map;
use crate::idastar::ida_star;
use crate::astar::a_star;
use pathfinding::prelude::idastar;
use std::time::Instant;

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

fn print_sol(sol: Option<(Vec<Map>, usize, usize, usize)>, time_elapsed: f64){
    match sol {
        Some((path, cost, time_c, space_c)) => {
            println!("Solution:");
            for i in 0..path.len() {
                for j in 0..path[i].size {
                    println!("{:?}", path[i].matrix[j]);
                }
                if i != path.len() - 1{
                    println!("    |");
                    println!("    V");
                }
            }
            println!("Number of movements: {}", cost);
            println!("Time complexity: {}", time_c);
            println!("Space complexity: {}", space_c);
            println!("Time elapsed: {}", time_elapsed);
        }
        None => println!("No solution found"),
    }
}


pub fn solver(heuristic: &str,algorithm: &str, h_method: &str,start: &Map, goal: &Map, max_p: usize){
    if algorithm == "default"{
        let result = idastar(start, |m| m.successors(), |m| m.manhattan_dist(goal),|m| *m == *goal);
        println!("Result: {:?}", result);
    }
    else{
        let time_start = Instant::now();
        let result = match algorithm {
            "ida_star" => ida_star(&start, &goal, str_to_heuristic(heuristic), str_to_method(h_method), &max_p),
            "a_star" => a_star(&start, &goal, str_to_heuristic(heuristic), str_to_method(h_method), &max_p),
            _ => panic!("Invalid algorithm"),
        };
        print_sol(result, time_start.elapsed().as_micros() as f64 / 1000000.0);
    }
   

}
