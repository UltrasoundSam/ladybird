use rand::Rng;
use std::collections::HashSet;

const N: i32 = 12;

fn run_iteration<R: Rng + ?Sized>(rng: &mut R) -> i32 {
    let mut current_location: i32 = 0;
    let mut visited: HashSet<i32> = HashSet::new();
    visited.insert(current_location);

    let directions = [-1, 1];

    while visited.len() < (N as usize - 1) {

        let step = directions[rng.random_range(0..2)];
        current_location = (current_location + step).rem_euclid(N);
	visited.insert(current_location);
    }

    for i in 0..N {
        if !visited.contains(&i) {
            return i;
        }
    }
    unreachable!("There must be exactly one unvisited element");
}



fn main() {
    let iterations = 1_000_000;

    // Create a iterator over the iterations
    let results: Vec<i32> = (0..iterations)
        .into_iter()
        .map(|_| {
            let mut rng = rand::rng();
            run_iteration(&mut rng)
        })
        .collect();

    // Count occurrences
    let mut counts = [0usize; 12];
    for v in results {
        counts[v as usize] += 1;
    }

    // Percentages (1–11, renormalized)
    let total_nonzero: usize = counts.iter().skip(1).sum();
    println!("Results (percentages, 1–11 sum to 100%):");
    for (i, c) in counts.iter().enumerate().skip(1) {
        let pct = (*c as f64) * 100.0 / (total_nonzero as f64);
        println!("{:2}: {:6.3}%", i, pct);
    }
}
