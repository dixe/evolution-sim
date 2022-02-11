use crate::basic_types::Coord;

#[derive(Debug, Default, Clone)]
pub struct GridTile {
    pub individual_index: Option<usize>,
    pub pheromone_level: u8,
}

impl GridTile {
    pub fn is_empty(&self) -> bool {
        // TODO: when adding walls also check if wall
        self.individual_index.is_none()
    }
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub tiles: Vec<GridTile>,
    pub size: Coord,
}

impl Grid {
    pub fn new(size: Coord) -> Self {
        Grid {
            tiles: vec![GridTile::default(); size.x * size.y],
            size,
        }
    }

    pub fn clear(&mut self) {
        for tile in &mut self.tiles {
            tile.individual_index = None;
            tile.pheromone_level = 0;
        }
    }

    pub fn increment_pheromone(&mut self, index: usize, inc: u8) {
        self.tiles[index].pheromone_level = self.tiles[index].pheromone_level.saturating_add(inc);
    }

    pub fn decrement_pheromone(&mut self, index: usize, mut dec: u8) {
        // max sub the current level
        dec = u8::min(self.tiles[index].pheromone_level, dec);

        self.tiles[index].pheromone_level -= dec;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn pheromones() {
        let mut grid = Grid::new(Coord { x: 10, y: 10 });

        let index = 15;
        let inc = 20;
        grid.increment_pheromone(index, inc);

        // increment once increments
        assert_eq!(inc, grid.tiles[index].pheromone_level);
        for _ in 0..300 {
            grid.increment_pheromone(index, 10);
        }

        // max at 255
        assert_eq!(255, grid.tiles[index].pheromone_level);

        grid.clear();
        assert_eq!(0, grid.tiles[index].pheromone_level);

        // DECREMENT
        let dec_index = index + 1;
        grid.increment_pheromone(dec_index, 10);
        grid.decrement_pheromone(dec_index, 5);

        assert_eq!(5, grid.tiles[dec_index].pheromone_level);

        grid.decrement_pheromone(dec_index, 10);
        grid.decrement_pheromone(dec_index, 10);

        assert_eq!(0, grid.tiles[dec_index].pheromone_level);
    }
}
