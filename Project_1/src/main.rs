use plotters::prelude::*;
use rand::Rng;
use std::error::Error;
use std::path::Path;
use csv::ReaderBuilder;

#[derive(Clone)]
pub struct Actor {
    pub bitstring: Vec<bool>, // Bitstring
}

pub struct Population {
    pub size: usize, // Population size
    pub actors: Vec<Actor>,
    pub mutation_rate: f64,
    pub data: Vec<(usize, usize, usize, usize)>, // Store CSV data
}

impl Actor {
    // Evaluate fitness based on the data rows selected by the bitstring
    pub fn fit(&self, data: &[(usize, usize, usize, usize)], max_size: i64) -> usize {
        // Calculate the total sum of 'p' where the bit is true
        let sum_p: i64 = self.bitstring.iter().enumerate()
            .filter_map(|(i, &bit)| if bit { Some(data[i].1 as i64) } else { None })
            .sum();

        // Calculate the total fitness by summarizing 'w' where the bit is true
        let total_fitness: usize = self.bitstring.iter().enumerate()
            .filter_map(|(i, &bit)| if bit { Some(data[i].2) } else { None })
            .sum();

        // Apply punishment if the sum of 'p' exceeds 'max_size'
        if sum_p > max_size {
            let penalty = ((sum_p - max_size) as usize).min(total_fitness); // Ensure penalty does not exceed total fitness
            total_fitness - penalty
        } else {
            total_fitness
        }
    }
}


impl Population {
    // Create a new Population with a given size and bitstring length
    pub fn new(size: usize, bitstring_length: usize, mutation_rate: f64, data: Vec<(usize, usize, usize, usize)>) -> Self {
        let actors = (0..size)
            .map(|_| Actor {
                bitstring: (0..bitstring_length)
                    .map(|_| rand::thread_rng().gen_bool(0.5))
                    .collect(),
            })
            .collect();

        Population {
            size,
            actors,
            mutation_rate,
            data,
        }
    }

    // Calculate selection probabilities for each actor
    pub fn calculate_chance(&self, total_space: i64) -> Vec<f64> {
        let mut total_fit: f64 = 0.0;
        let mut prob: Vec<f64> = vec![0.0; self.size];

        for (i, actor) in self.actors.iter().enumerate() {
            let fitness = actor.fit(&self.data, total_space) as f64; // Calculate fitness
            total_fit += fitness; // Accumulate total fitness
            prob[i] = fitness; // Store fitness in prob temporarily
        }

        for value in prob.iter_mut() {
            *value /= total_fit; // Normalize fitness to get selection probability
        }

        prob
    }

    // Perform roulette selection to create the next generation
    pub fn roulette_selection(&mut self, total_space: i64) {
        let prob = self.calculate_chance(total_space);
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
            let rand_val: f64 = rng.gen_range(0.0..1.0); // Random number between 0 and 1
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
                    *bit = !*bit; // Flip the bit
                }
            }
        }
    }

    pub fn apply_crossover(&mut self) {
        let mut rng = rand::thread_rng();
        
        // Iterate over pairs of actors
        let actors_len = self.actors.len();
        for i in (0..actors_len).step_by(2) {
            if i + 1 < actors_len {
                let crossover_point = rng.gen_range(0..self.actors[0].bitstring.len()); // Random crossover point
                
                // Use split_at_mut to borrow two non-overlapping mutable slices
                let (actor1, actor2) = self.actors.split_at_mut(i + 1); // Split the slice at the index i+1
                let (actor1, actor2) = ( &mut actor1[i], &mut actor2[0]);
    
                // Perform crossover
                for j in crossover_point..actor1.bitstring.len() {
                    std::mem::swap(&mut actor1.bitstring[j], &mut actor2.bitstring[j]);
                }
            }
        }
    }
}

fn read_csv<P: AsRef<Path>>(path: P) -> Result<Vec<(usize, usize, usize, usize)>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;
    let mut data = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let i: usize = record[0].parse()?;
        let p: usize = record[1].parse()?;
        let w: usize = record[2].parse()?;
        let x: usize = record[3].parse()?;
        data.push((i, p, w, x));
    }
    Ok(data)
}

fn main() {
    let total_space: i64 = 280785;
    let data = read_csv("data/KP/knapPI_12_500_1000_82.csv").expect("Failed to read CSV");
    let bitstring_length = data.len();
    let mutation_rate = 1f64 / (bitstring_length as f64);
    let mut population = Population::new(500, bitstring_length, mutation_rate, data);

    let mut generation_fitness: Vec<usize> = Vec::new();

    // Run the simulation for 500 generations
    for _run in 1..=1000 {
        population.roulette_selection(total_space);
        population.apply_crossover();
        population.mutate_population();

        // Calculate the average fitness of the current population
        let avg_fitness: usize = population.actors.iter()
            .map(|actor| actor.fit(&population.data, total_space))
            .sum::<usize>() / population.size;
        
        generation_fitness.push(avg_fitness);
    }

    // Plotting
    let root_area = BitMapBackend::new("fitness_plot.png", (640, 480)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root_area)
        .caption("Average Fitness Over Generations", ("sans-serif", 50))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(0..500, 0..*generation_fitness.iter().max().unwrap() + 10)
        .unwrap();

    chart.configure_mesh().draw().unwrap();
    chart.draw_series(LineSeries::new(
        (0..).zip(generation_fitness.iter()).map(|(x, y)| (x, *y)),
        &RED,
    )).unwrap();
}