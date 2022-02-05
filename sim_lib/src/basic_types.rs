#[derive(Debug, Clone, Copy)]
pub struct Gene {
    pub from_neuron: u8, // First all the inputs are setup, then all internal neuron
    pub to_neuron: u8, // first come all internal neurons then action neurons
    pub weight: i16, //  weight of connection
}


//pub type ActionNeuron = fn(f64);
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Action {
    MoveForward,
    MoveX,
    MoveY,

    EmitPheromone,

    SetOscPeriod,

    SetResponsivness,
}

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
