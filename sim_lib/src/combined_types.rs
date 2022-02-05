use crate::basic_types::*;
use rand;
use std::fmt;
use crate::sensor_neurons;

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
        Self {
            genome: vec![],
            grid_index: 0,
            forward: Dir::Up,
            index: 0
        }
    }
}

#[derive(Debug, Clone)]
pub struct World {
    pub grid: Grid,
    pub individuals: Vec::<Individual>,
}

impl World {
    pub fn new(size: Coord) -> Self {

        let mut tiles = Vec::new();
        for i in 0..size.y {
            for j in 0..size.x {
                tiles.push(GridTile::default());
            }
        }

        World {
            grid: Grid { tiles, size },
            individuals: Vec::new()
        }
    }

    pub fn is_dir_empty(&self, grid_index: usize, dir: Dir) -> bool {

        match dir {
            Dir::Up => {

                // subtract world.width if index > width
                if grid_index < self.grid.size.x {
                    return false;
                }

                return self.grid.tiles[ grid_index - self.grid.size.x].is_empty();
            },
            Dir::Down => {
                // subtract world.width if index > width
                if grid_index > self.grid.size.y - 1 * self.grid.size.x {
                    return false;
                }

                return self.grid.tiles[ grid_index + self.grid.size.x].is_empty();
            }
            Dir::Left => {
                // subtract world.width if index > width
                if grid_index % self.grid.size.x == 0 {
                    return false;
                }

                return self.grid.tiles[ grid_index - 1 ].is_empty();
            }

            Dir::Right => {
                // subtract world.width if index > width
                if grid_index % self.grid.size.x == self.grid.size.x - 1 {
                    return false;
                }

                return self.grid.tiles[ grid_index + 1 ].is_empty();
            }
        }
    }

    pub fn move_indiv_dir(&mut self, indiv_index: usize, dir: Dir) {
        let old_index = self.individuals[indiv_index].grid_index;

        // If we are blocked by a wall or another idividual we cannot move
        if !self.is_dir_empty(old_index, dir) {
            return
        }

        // calc new grid index
        let new_index = match dir {
            Dir::Right => old_index + 1,
            Dir::Left => old_index -1,
            Dir::Up => old_index - self.grid.size.x,
            Dir::Down => old_index + self.grid.size.x,
        };


        // update the individuals grid index
        self.individuals[indiv_index].grid_index = new_index;

        // update the grid
        self.grid.tiles[old_index].individual_index = None;
        self.grid.tiles[new_index].individual_index = Some(indiv_index);

    }

    pub fn add_individual(&mut self, mut indiv: Individual) -> usize{
        let index = self.individuals.len();
        indiv.index = index;

        self.grid.tiles[indiv.grid_index].individual_index = Some(indiv.index);
        self.individuals.push(indiv);

        index
    }

    pub fn reset(&mut self, indivs: Vec::<Individual>) {
        // clear grid
        self.grid.clear();
        self.individuals = indivs;

        // Update world with individual pos
        for indiv in &mut self.individuals {
            if self.grid.tiles[indiv.grid_index].individual_index != None {
                panic!("\nInsertering indiv into grid index already occupied: Indiv:\n '{0:#?}'", indiv);
            }

            self.grid.tiles[indiv.grid_index].individual_index = Some(indiv.index);
        }
    }
}



#[derive(Debug, Clone)]
pub struct Configuration {
    pub sensor_neurons: Vec::<Sensor>,
    pub hidden_neurons: usize,
    pub action_neurons: Vec::<Action>,
    pub generation_steps: usize,
}


impl Default for Configuration {
    fn default () -> Self {
        Configuration {
            sensor_neurons: all_sensors(),
            hidden_neurons: 2,
            action_neurons: all_actions(),
            generation_steps: 300
        }
    }
}
