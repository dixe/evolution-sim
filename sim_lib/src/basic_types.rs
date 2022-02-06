
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




#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn gene_bit_size() {
        assert_eq!(GENE_BITS, 8 * std::mem::size_of::<Gene>());
    }

}
