
pub type Genome = Vec::<Gene>;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Gene {
    pub from_neuron: u8, // First all the inputs are setup, then all internal neuron
    pub to_neuron: u8, // first come all internal neurons then action neurons
    pub weight: i16, //  weight of connection
}



macro_rules! all_variants {
    ($typ:ty) => {

    }
}

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
        static $array: &[$name] = &[
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


#[derive(Debug, Default, Clone)]
pub struct GridTile {
    pub individual_index: Option<usize>,
    pub pheromon_level: u8
}

impl GridTile {

    pub fn is_empty(&self) -> bool {
        // TODO: when adding walls also check if wall
        self.individual_index.is_none()

    }
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub tiles: Vec::<GridTile>,
    pub size: Coord
}

impl Grid {
    pub fn clear(&mut self) {

        for tile in &mut self.tiles {
            tile.individual_index = None;
            tile.pheromon_level = 0;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right
}
