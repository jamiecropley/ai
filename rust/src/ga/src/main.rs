use rand::Rng;

/// Perform single-point crossover between two parent bit-strings.
fn crossover(parent_a: &str, parent_b: &str) -> String {
    let len = parent_a.len();
    let split = rand::rng().random_range(0..len);
    let prefix: String = parent_a.chars().take(split).collect();
    let suffix: String = parent_b.chars().skip(split).collect();
    prefix + &suffix
}

/// Compute fitness as fraction of '1's in the bit-string.
fn fitness(individual: &str) -> f64 {
    let count = individual.chars().filter(|&c| c == '1').count();
    count as f64 / individual.len() as f64
}

/// Mutate the individual by flipping a random bit with 50% probability.
fn mutate(individual: &str) -> String {
    let mut rng = rand::rng();
    if rng.random_bool(0.5) {
        // No mutation
        individual.to_string()
    } else {
        let mut chars: Vec<char> = individual.chars().collect();
        let idx = rng.random_range(0..chars.len());
        chars[idx] = if chars[idx] == '0' { '1' } else { '0' };
        chars.into_iter().collect()
    }
}

fn main() {
    // Initial population of 4 individuals (8-bit strings)
    let mut population = vec![
        String::from("00000000"),
        String::from("00000010"),
        String::from("00001000"),
        String::from("00100001"),
    ];

    let mut generation = 0;

    loop {
        // Print current generation and individuals
        println!("Gen {}: {:?}", generation, population);

        // Evaluate fitness and check for maximum
        let fitnesses: Vec<f64> = population.iter().map(|ind| fitness(ind)).collect();
        if let Some(pos) = fitnesses.iter().position(|&f| (f - 1.0).abs() < f64::EPSILON) {
            println!("Maximum fitness reached in individual {} at generation {}!", pos, generation);
            break;
        }

        // Find indices of two best parents
        let mut sorted: Vec<(usize, &f64)> = fitnesses.iter().enumerate().collect();
        sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
        let best_idx = sorted[0].0;
        let second_idx = sorted[1].0;

        // Reproduce new generation
        let parent_a = population[best_idx].clone();
        let parent_b = population[second_idx].clone();
        population = (0..population.len())
            .map(|_| mutate(&crossover(&parent_a, &parent_b)))
            .collect();

        generation += 1;
    }
}
