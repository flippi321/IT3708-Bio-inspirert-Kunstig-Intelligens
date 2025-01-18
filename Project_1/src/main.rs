use rand::Rng;

#[derive(Clone)]
pub struct Actor {
    pub bitstring: Vec<bool>,  // Bitstring
}

pub struct Population {
    pub size: usize,        // Population size
    pub actors: Vec<Actor>,
    pub mutation_rate: f64,
}

impl Actor {
    // Evaluate fitness
    pub fn fit(&self) -> usize {
        // Example fitness function: count the number of `true` bits
        self.bitstring.iter().filter(|&&bit| bit).count()
    }
}

impl Population {
    // Create a new Population with a given size and bitstring length
    pub fn new(size: usize, bitstring_length: usize, mutation_rate: f64) -> Self {
        let actors = (0..size)
            .map(|_| Actor {
                bitstring: (0..bitstring_length).map(|_| rand::thread_rng().gen_bool(0.5)).collect(),
            })
            .collect();
        
        Population { size, actors, mutation_rate }
    }

    // Calculate selection probabilities for each actor
    pub fn calculate_chance(&self) -> Vec<f64> {
        let mut total_fit: f64 = 0.0;
        let mut prob: Vec<f64> = vec![0.0; self.size];

        // Summarize all Actors
        for (i, actor) in self.actors.iter().enumerate() {
            let fitness = actor.fit() as f64;  // Calculate fitness
            total_fit += fitness;              // Accumulate total fitness
            prob[i] = fitness;                 // Store fitness in prob temporarily
        }

        // Calculate the probability of selection for each actor
        for value in prob.iter_mut() {
            *value /= total_fit;  // Normalize fitness to get selection probability
        }
        
        prob
    }

    // Perform roulette selection to create the next generation
    pub fn roulette_selection(&mut self) {
        let prob = self.calculate_chance();
        let mut rng = rand::thread_rng();
        let mut new_actors = Vec::with_capacity(self.size);

        // Create a cumulative probability distribution
        let mut cumulative_prob: Vec<f64> = vec![0.0; self.size];
        cumulative_prob[0] = prob[0];
        for i in 1..self.size {
            cumulative_prob[i] = cumulative_prob[i - 1] + prob[i];
        }

        // Select actors based on the cumulative probability distribution
        for _ in 0..self.size {
            let rand_val: f64 = rng.gen_range(0.0..1.0);  // Random number between 0 and 1
            // Find the selected actor based on the random value
            let selected_actor_index = cumulative_prob.iter().position(|&c| c >= rand_val).unwrap();
            new_actors.push(self.actors[selected_actor_index].clone());
        }

        // Update the population with the new generation of actors
        self.actors = new_actors;
    }

    pub fn mutate_population(&mut self) {
        for actor in &mut self.actors {
            // Traverse each bit in the actor's bitstring
            for bit in &mut actor.bitstring {
                // Check if we shoul mutate
                if rand::thread_rng().gen_bool(self.mutation_rate) {
                    *bit = !*bit;  // Flip the bit
                }
            }
        }
    }
    
}

fn main() {
    // Create a population with 5 actors, each having a bitstring of length 10, and mutaion rate of 0.05
    let mutation_rate = 1f64 / (5f64 * 10f64);
    let mut population = Population::new(20, 10, mutation_rate);

    // Print each Actor's bitstring
    for (i, actor) in population.actors.iter().enumerate() {
        let bitstring: String = actor.bitstring.iter()
            .map(|&bit| if bit { '1' } else { '0' })
            .collect();
        println!("Actor {}: {}, eval: {}", i + 1, bitstring, actor.fit());
    }

    // Run roulette selection 5 times and print the selected actors for each run
    println!("\nRoulette Selection (5 times):");
    for run in 1..=500 {
        println!("\nRun {}:", run);
        population.roulette_selection();
        population.mutate_population();

        // Print the new population's bitstrings after roulette selection
        for (i, actor) in population.actors.iter().enumerate() {
            let bitstring: String = actor.bitstring.iter()
                .map(|&bit| if bit { '1' } else { '0' })
                .collect();
            println!("Actor {}: {}, fit: {}", i + 1, bitstring, actor.fit());
        }
    }
}
