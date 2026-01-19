use rand::Rng;
use std::collections::HashSet;
use rayon::prelude::*;


const N: i32 = 12;

fn run_iteration<R: Rng + ?Sized>(rng: &mut R) -> (i32, usize) {
    let mut current_location: i32 = 0;
    let mut visited: HashSet<i32> = HashSet::new();
    visited.insert(current_location);
    let mut steps = 0;
    let directions = [-1, 1];

    while visited.len() < (N as usize - 1) {

        let step = directions[rng.random_range(0..2)];
        current_location = (current_location + step).rem_euclid(N);
	    visited.insert(current_location);
        steps += 1;
    }

    let last_item = (0..N).find(|i| !visited.contains(i)).unwrap();
    (last_item, steps)
}


fn main() {
    let iterations = 1_000_000;

    // Create a iterator over the iterations
    let results: Vec<(i32, usize)> = (0..iterations)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::rng();
            run_iteration(&mut rng)
        })
        .collect();

    // Count occurrences
    let mut counts = [0usize; 12];
    let mut steps_total: usize = 0;

    for (last, steps) in results {
        counts[last as usize] += 1;
        steps_total += steps;
    }

    // Percentages (1–11, renormalized)
    let total_nonzero: usize = counts.iter().skip(1).sum();
    println!("Results (percentages, 1–11 sum to 100%):");
    for (i, c) in counts.iter().enumerate().skip(1) {
        let pct = (*c as f64) * 100.0 / (total_nonzero as f64);
        println!("{:2}: {:6.3}%", i, pct);
    }

    let average_steps = steps_total as f64 / iterations as f64;
    println!(
        "\nAverage steps before only one number remains unvisited: {:.2}",
        average_steps
    );
}
