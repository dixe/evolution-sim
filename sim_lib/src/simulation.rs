use rand::seq::SliceRandom;

use crate::combined_types::*;
use crate::basic_types::*;
use crate::network;
use crate::gene_functions;
use crate::sensor_neurons;
use crate::action_neurons;


#[derive(Clone)]
pub struct SimulationBuilder {
    sim: Simulation
}

impl SimulationBuilder {

    pub fn new(width: usize, height: usize) -> Self {
        Self {
            sim: Simulation::new(width, height)
        }
    }

    pub fn build(mut self) -> Simulation {
        for i in 0..self.sim.population_size {
            self.sim.brains.push(Brain {
                indiv_index: i,
                network: network::Network::empty()
            });
        }

        self.sim
    }

    pub fn population_size(&mut self, pop: usize) {
        self.sim.population_size = pop;

    }

    pub fn config_mut(&mut self) -> &mut Configuration {
        &mut self.sim.config
    }

}

#[derive(Clone)]
pub struct Simulation {
    config: Configuration,
    world: World,
    brains: Vec::<Brain>,

    generation: usize,
    generation_step: usize,

    rng: rand::rngs::ThreadRng,

    population_size: usize,
    genome_length: usize,


    individual_grid_placement_function: fn(&World, &mut Vec::<Individual>, &mut rand::rngs::ThreadRng)
}



#[derive(Debug, Clone)]
struct Brain {
    indiv_index: usize,
    network: network::Network
}


fn set_individual_grid_index_random(world: &World, indivs: &mut Vec::<Individual>, rng: &mut rand::rngs::ThreadRng) {
    // Place individuals randomly on the map
    let mut grid_indicies: Vec::<usize> = (0..world.grid.size.x * world.grid.size.y).collect();
    grid_indicies.shuffle(rng);

    for i in 0..indivs.len() {
        indivs[i].grid_index = grid_indicies[i];
    }
}

impl Simulation {
    fn new(width: usize, height: usize) -> Self {
        let mut brains = vec![];
        let population_size = 100;

        Self {
            config: Configuration::default(),
            world: World::new( Coord { x:128, y:128 }),
            brains,
            population_size,
            genome_length: 3,
            generation: 0,
            generation_step: 0,
            rng: rand::thread_rng(),
            individual_grid_placement_function: set_individual_grid_index_random
        }
    }


    pub fn initialize_first_generation(&mut self, initial_genome_func: Option<gene_functions::GenomeFunc<rand::rngs::ThreadRng>>) {

        let mut indivs = vec![];
        // generate individuals
        for i in 0..self.population_size {
            let mut genome = match initial_genome_func {
                Some(f) => f(&mut self.rng, self.genome_length),
                None => gene_functions::random_genome(&mut self.rng, self.genome_length)
            };
            let mut indiv = Individual::new();
            indiv.genome = genome;
            indivs.push(indiv);
        }


        for i in 0..indivs.len() {
            self.brains[i].network.initialize_from_genome(&indivs[i].genome, &self.config);
        }


        (self.individual_grid_placement_function)(&self.world, &mut indivs, &mut self.rng);


        // Set individuals in the world
        self.world.set_individuals(indivs);

    }



    pub fn generation(&self) -> usize {
        self.generation
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn run_generation(&mut self) {

        let gen = self.generation;

        while gen == self.generation {
            self.step_single_thread();
        }
    }

    pub fn step_single_thread(&mut self) {
        for brain in &mut self.brains {
            for action in brain.network.run(&self.config.sensor_neurons, &self.world, &self.world.individuals[brain.indiv_index]) {
                action_neurons::perform_action(action, &mut self.world, brain.indiv_index);
            }
        }

        self.generation_step += 1;

        if self.generation_step >= self.config.generation_steps {
            self.generation += 1;
            self.generation_step = 0;
        }

        // TODO: initialize new generation
    }

}


#[cfg(test)]
mod tests {

    use super::*;
    use crate::sensor_neurons::*;
    use crate::action_neurons::*;

    /// Test that a single a simple brain of move forward, move an indiviual up in a single step
    #[test]
    fn forward_sim_1_step() {

        let mut builder = SimulationBuilder::new(128, 128);

        builder.config_mut().sensor_neurons = vec![constant_neuron];
        builder.config_mut().hidden_neurons = 0;
        builder.config_mut().action_neurons = vec![Action::MoveForward];
        builder.population_size(1);

        let mut sim = builder.build();
        sim.initialize_first_generation(Some(|rng, genome_len| gene_functions::fixed_genome(rng, genome_len, 0, 0)));

        // find first indiv with index > width (128)

        // move current individual to grid index 300, to make sure that there is space above for a move.


        let mut index = 0;
        let mut grid_index = 0;



        for indiv in &sim.world.individuals {

            if indiv.grid_index > 128 && indiv.forward == Dir::Up {
                index = indiv.index;
                grid_index = indiv.grid_index;
                break;
            }

        }

        assert!(grid_index != 0);

        sim.step_single_thread();

        assert_eq!(grid_index - 128, sim.world.individuals[index].grid_index);

    }


}
