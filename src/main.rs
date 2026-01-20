use rand::Rng;
use std::collections::HashSet;
use rayon::prelude::*;


const NUM_HOURS: i32 = 12;

fn run_iteration<R: Rng + ?Sized>(rng: &mut R) -> (i32, usize) {
    let mut current_location: i32 = 0;

    // Keep track of visited numbers with hashset
    let mut visited: HashSet<i32> = HashSet::new();
    visited.insert(current_location);

    // Also want to know how many steps we have taken before we stop
    let mut steps = 0;

    let directions = [-1, 1];

    // Keep random walking (50 % chance moving + or -) until we have visited 11 separate
    // locations
    while visited.len() < (NUM_HOURS as usize - 1) {
        let step = directions[rng.random_range(0..2)];
        current_location = (current_location + step).rem_euclid(NUM_HOURS);
	    visited.insert(current_location);
        steps += 1;
    }

    // Now we have stopped, let's find which value is i is NOT in hashset
    let last_item = (0..NUM_HOURS).find(|i| !visited.contains(i)).unwrap();
    (last_item, steps)
}


fn main() {
    let iterations = 1_000_000;

    // Use parallel looping since everything is independent
    let results: Vec<(i32, usize)> = (0..iterations)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::rng();
            run_iteration(&mut rng)
        })
        .collect();

    // Count occurrences of each stopping number and length of each simulation
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

    // Calculate average steps
    let average_steps: f64 = steps_total as f64 / iterations as f64;
    println!(
        "\nAverage steps before only one number remains unvisited: {:.2}",
        average_steps
    );
}
