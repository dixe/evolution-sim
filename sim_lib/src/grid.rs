use crate::basic_types::Coord;

#[derive(Debug, Default, Clone)]
pub struct GridTile {
    pub individual_index: Option<usize>,
    pub pheromone_level: u8
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

    pub fn new(size: Coord) -> Self {

        Grid {
            tiles :vec![GridTile::default(); size.x * size.y],
            size
        }
    }


    pub fn clear(&mut self) {

        for tile in &mut self.tiles {
            tile.individual_index = None;
            tile.pheromone_level = 0;
        }
    }

    pub fn increment_pheromone(&mut self, index: usize, inc: u8 ) {
        self.tiles[index].pheromone_level = self.tiles[index].pheromone_level.saturating_add(inc);
    }
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn pheromones() {

        let mut grid = Grid::new(Coord{ x:10, y:10});

    }
}
