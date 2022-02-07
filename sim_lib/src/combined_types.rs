use crate::basic_types::*;
use crate::grid::*;


#[derive(Debug, Clone)]
pub struct World {
    pub grid: Grid,
    pub individuals: Vec::<Individual>,
}

impl World {
    pub fn new(size: Coord) -> Self {

        World {
            grid: Grid::new(size),
            individuals: Vec::new()
        }
    }

    pub fn is_dir_empty(&self, grid_index: usize, dir: Dir) -> bool {

        match dir {
            Dir::Up => {
                if grid_index < self.grid.size.x {
                    return false;
                }

                return self.grid.tiles[ grid_index - self.grid.size.x].is_empty();
            },
            Dir::Down => {

                let next_index = grid_index + self.grid.size.x;

                if next_index >= self.grid.size.y * self.grid.size.x {
                    return false;
                }

                return self.grid.tiles[next_index].is_empty();
            }
            Dir::Left => {
                if grid_index % self.grid.size.x == 0 {
                    return false;
                }

                return self.grid.tiles[ grid_index - 1 ].is_empty();
            }

            Dir::Right => {
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
