use rand;
use rand::Rng;
use crate::survival_criteria as sc;

pub type Genome = Vec::<Gene>;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Gene {
    pub from_neuron: u8, // First all the inputs are setup, then all internal neuron
    pub to_neuron: u8, // first come all internal neurons then action neurons
    pub weight: i16, //  weight of connection
}

pub static GENE_BITS : usize = 32;

macro_rules! make_enum {
    (
        $name:ident $array:ident {
            $( $variant:ident, )*
        }
    ) => {
        #[derive(Debug, PartialEq, Clone, Copy)]
        pub enum $name {
            $( $variant, )*
        }
        pub static $array: &[$name] = &[
            $( $name::$variant, )*
        ];
    }
}

// SENSORS (inputs)
pub fn all_sensors() -> Vec::<Sensor> {
    ALL_SENSORS.to_vec()
}

make_enum! (Sensor ALL_SENSORS {
    // Location
    LocY,
    LocX,

    WorldBorderDistX,
    WorldBorderDistY,

    BlockedForward,

    Random,
    Constant,

});



#[derive(Debug, Clone)]
pub struct Individual {
    pub genome: Genome,
    // brain not stored here, but on simulation
    pub grid_index: usize,
    pub index: usize,
    pub forward: Dir
}

impl Individual {

    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            genome: vec![],
            grid_index: 0,
            forward: ALL_DIRS[rng.gen_range(0..ALL_DIRS.len())],
            index: 0
        }
    }
}



//ACTIONS (output)
pub fn all_actions() -> Vec::<Action> {
    ALL_ACTIONS.to_vec()
}

make_enum! (Action ALL_ACTIONS {
    MoveForward,
    MoveX,
    MoveY,

    EmitPheromone,

    SetOscPeriod,

    SetResponsivness,

});


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coord {
    pub x: usize,
    pub y: usize
}


#[derive(Debug, Clone, Copy)]
pub struct Activation {
    pub action: Action,
    pub weight: f64,
    pub indiv_index: usize
}


make_enum! (Dir ALL_DIRS {
    Up,
    Down,
    Left,
    Right,
});





#[derive(Debug, Clone, Copy)]
pub struct Configuration {
    pub hidden_neurons: usize,
    pub generation_steps: usize,
    pub mutation_rate: f32,
    pub population_size: usize,
    pub genome_length: usize,
    pub criteria: sc::SurvivalCriteria,

}


impl Default for Configuration {
    fn default () -> Self {
        Configuration {
            criteria: sc::SurvivalCriteria::Border(0.010),
            genome_length: 24,
            population_size: 1000,
            hidden_neurons: 5,
            generation_steps: 300,
            mutation_rate: 0.0,
        }
    }
}




#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn gene_bit_size() {
        assert_eq!(GENE_BITS, 8 * std::mem::size_of::<Gene>());
    }

}
