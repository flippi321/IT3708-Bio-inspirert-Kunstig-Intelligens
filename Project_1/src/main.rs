use rand::Rng; 

pub struct Actor {
    pub bitstring: Vec<bool>,  // Bitstring
}

pub struct Population {
    pub size: usize,        // Apparantly better than integer
    pub actors: Vec<Actor>,
}

impl Population {
    // Create a new Population with a given size and bitstring length
    pub fn new(size: usize, bitstring_length: usize) -> Self {
        let actors = (0..size)
            .map(|_| Actor {
                bitstring: (0..bitstring_length).map(|_| rand::thread_rng().gen_bool(0.5)).collect(),
            })
            .collect();
        
        Population { size, actors }
    }
}

fn main() {
    // Create a population with 5 actors, each having a bitstring of length 10
    let population = Population::new(5, 10);

    // Print each Actor's bitstring
    for (i, actor) in population.actors.iter().enumerate() {
        let bitstring: String = actor.bitstring.iter()
            .map(|&bit| if bit { '1' } else { '0' })
            .collect();
        println!("Actor {}: {}", i + 1, bitstring);
    }
}