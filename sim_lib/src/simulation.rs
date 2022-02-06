use rand::seq::SliceRandom;
use rand::Rng;
use rayon::prelude::*;

use crate::combined_types::*;
use crate::basic_types::*;
use crate::network;
use crate::gene_functions;
use crate::action_neurons;
use crate::survival_criteria as sc;

#[derive(Clone)]
pub struct SimulationBuilder {
    sim: Simulation,

}

impl SimulationBuilder {

    pub fn new(width: usize, height: usize) -> Self {
        Self {
            sim: Simulation::new(width, height),

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

    pub fn criteria(mut self, c: sc::SurvivalCriteria) -> Self
    {
        self.sim.criteria = c;
        self
    }

    pub fn population_size(mut self, pop: usize) -> Self {
        self.sim.population_size = pop;
        self
    }

    pub fn mutation_rate(mut self, rate: f32) -> Self {
        self.sim.config.mutation_rate = rate;
        self
    }

    pub fn action_neurons(mut self, an: Vec::<Action>) -> Self {
        self.sim.config.action_neurons = an;
        self
    }

    pub fn hidden_neurons(mut self, hn: usize) -> Self {
        self.sim.config.hidden_neurons = hn;
        self
    }

    pub fn sensor_neurons(mut self, sn: Vec::<Sensor>) -> Self {
        self.sim.config.sensor_neurons = sn;
        self
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

    criteria: sc::SurvivalCriteria,

    individual_grid_placement_function: fn(&World, &mut Vec::<Individual>, &mut rand::rngs::ThreadRng),

    stats: Vec::<GenerationStats>


}


#[derive(Debug, Clone, Copy, Default)]
struct GenerationStats {
    survival_rate: f32
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
        let brains = vec![];
        let population_size = 1000;

        Self {
            config: Configuration::default(),
            world: World::new( Coord { x: width, y: height }),
            brains,
            population_size,
            genome_length: 3,
            generation: 0,
            generation_step: 0,
            rng: rand::thread_rng(),
            criteria: sc::SurvivalCriteria::BottomPart(0.10),
            individual_grid_placement_function: set_individual_grid_index_random,
            stats: vec![Default::default()]
        }
    }


    pub fn initialize_first_generation(&mut self, initial_genome_func: Option<gene_functions::GenomeFunc<rand::rngs::ThreadRng>>) {

        let mut indivs = vec![];
        // generate individuals
        for i in 0..self.population_size {
            let  genome = match initial_genome_func {
                Some(f) => f(&mut self.rng, self.genome_length),
                None => gene_functions::random_genome(&mut self.rng, self.genome_length)
            };
            let mut indiv = Individual::new();
            indiv.index = i;
            indiv.genome = genome;
            indivs.push(indiv);
        }

        self.setup_individuals(indivs);
    }


    fn setup_individuals(&mut self, mut indivs: Vec::<Individual>) {
        for i in 0..indivs.len() {
            self.brains[i].network.initialize_from_genome(&indivs[i].genome, &self.config);
        }

        (self.individual_grid_placement_function)(&self.world, &mut indivs, &mut self.rng);

        // Set individuals in the world
        self.world.reset(indivs);
    }


    pub fn generation(&self) -> usize {
        self.generation
    }

    pub fn world(&self) -> &World {
        &self.world
    }


    pub fn population_count(&self) -> usize {
        self.world.individuals.len()
    }


    pub fn last_survival_rate(&mut self) -> f32 {

        let mut index = self.generation;
        if self.generation_step < self.config.generation_steps && index > 0 {
            index -= 1;
        }

        if index == self.generation {
            // Update the generation stats for current gen, to make sure it is computed
            let survive_indexes = sc::surviving_indexes(&self.world, self.criteria);
            self.stats[self.generation].survival_rate = (survive_indexes.len() as f32 / self.population_size as f32) * 100.0;
        }

        self.stats[index].survival_rate
    }

    pub fn survive_cells(&self) -> Vec::<Coord>{
        sc::survive_cells(&self.world, self.criteria)

    }

    pub fn run_generation(&mut self) {

        let gen = self.generation;

        while gen == self.generation {
            self.step_single_thread();
        }
    }

    /// Run a single step of the simulation.
    /// Return bool as to if this was the last step of current gen.
    /// If it was last step, next call to step_single will initialize a new generation
    pub fn step_single_thread(&mut self) -> bool {


        if self.generation_step >= self.config.generation_steps {
            self.generation += 1;
            self.generation_step = 0;


            // Update the generation stats for current
            let survive_indexes = sc::surviving_indexes(&self.world, self.criteria);
            self.stats[self.generation -1].survival_rate = (survive_indexes.len() as f32 / self.population_size as f32) * 100.0;


            let mut new_indivs = vec![];

            for i in 0..self.population_size {
                let index = survive_indexes[self.rng.gen_range(0..survive_indexes.len())];

                let mut indiv = Individual::new();
                let mutation_change: f32 = self.rng.gen();

                indiv.genome = self.world.individuals[index].genome.clone();
                if mutation_change < self.config.mutation_rate {
                    gene_functions::mutate_genome(&mut self.rng, &mut indiv.genome);
                }
                // TODO: also maybe mutate genome

                indiv.index = i;

                new_indivs.push(indiv);
            }

            self.setup_individuals(new_indivs);
        }

        let brains = &mut self.brains;
        let sensors = &self.config.sensor_neurons;
        let world = &self.world;
        let activations: Vec::<Vec::<Activation>> = brains.par_iter_mut()
            .map(|brain|{

                let indiv = &world.individuals[brain.indiv_index];

                brain.network.run(sensors, world, indiv)
            })
            .collect();


        for indiv_activations in &activations {
            for activation in indiv_activations {
                action_neurons::perform_action(activation, &mut self.world);
            }
        }



        self.generation_step += 1;
        self.stats.push(Default::default());

        self.generation_step >= self.config.generation_steps
    }

    pub fn reset_generation(&mut self) {
        self.generation_step = 0;
        self.setup_individuals(self.world.individuals.clone());
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

        let mut sim = SimulationBuilder::new(128, 128)
            .sensor_neurons(vec![Sensor::Constant])
            .hidden_neurons(0)
            .action_neurons(vec![Action::MoveForward])
            .population_size(1)
            .build();

        sim.initialize_first_generation(Some(|rng, genome_len| gene_functions::fixed_genome(rng, genome_len, 0, 0)));

        // find first indiv with index > width (128)

        // move current individual to grid index 300, to make sure that there is space above for a move.


        let mut index = 0;
        let mut grid_index = 0;

        let genome = sim.world.individuals[0].genome.clone();
        let mut indiv = Individual::new();

        indiv.genome = genome;
        indiv.forward = Dir::Down;

        let start = 128;
        indiv.grid_index = start;

        sim.world.reset(vec![indiv]);

        sim.step_single_thread();

        assert_eq!(start + 128, sim.world.individuals[0].grid_index);
    }
}
