use std::collections::HashSet;

use crate::board::Board;

pub fn search(start: Board) -> Vec<Board> {
    let mut visited: HashSet<Board> = HashSet::new();

    let mut round_sets: Vec<HashSet<Board>> = Vec::new();
    let mut next: HashSet<Board> = HashSet::new();
    next.insert(start);
    for round in 0..120 {
        println!("Round {}, Visited: {}, Queue: {}", round, visited.len(), next.len());
        let mut new_next = HashSet::new();
        for b in next.iter() {
            if b.is_solved() {
                return process_sln(round_sets, b.clone());
            }
            visited.insert(b.clone());
            for n in b.follow() {
                if !visited.contains(&n) && !next.contains(&n) {
                    new_next.insert(n);
                }
            }
        }
        round_sets.push(next);
        next = new_next;
    }

    panic!("No solution found");

    fn process_sln(round_sets: Vec<HashSet<Board>>, sln: Board) -> Vec<Board> {
        let mut sln_vec = Vec::new();
        let mut sln = sln;
        for round in (0..round_sets.len()).rev() {
            let round_set = &round_sets[round];
            for b in round_set.iter() {
                if b.follow().contains(&sln) {
                    sln_vec.push(sln);
                    sln = b.clone();
                    break;
                }
            }
        }
        sln_vec.push(sln);
        sln_vec
    }
}