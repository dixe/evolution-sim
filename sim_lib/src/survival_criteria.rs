use crate::combined_types::*;
use crate::basic_types::*;
use crate::index_functions::*;


#[derive(Debug, Clone, Copy)]
pub enum SurvivalCriteria {
    TopPart(f32)
}


pub fn surviving_indexes(world: &World, criteria: SurvivalCriteria) -> Vec::<usize> {

    match criteria {
        SurvivalCriteria::TopPart(percentage) => {
            survive_top(world, percentage)
        }
    }
}

fn survive_top(world: &World, pct: f32) -> Vec::<usize> {

    let max_survive_y = (world.grid.size.y as f32 * pct) as usize;

    let mut res = vec![];

    for (i, indiv) in world.individuals.iter().enumerate() {
        let coord = index_to_coord(indiv.grid_index, world.grid.size);
        if coord.y < max_survive_y {
            res.push(i);
        }
    }

    res


}

#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn survive_top_none() {

        let mut world = World::new(Coord {x: 128, y: 128});
        let mut indiv = Individual::new();

        indiv.grid_index = 64* 128;

        world.add_individual(indiv);

        let indexes = surviving_indexes(&world, SurvivalCriteria::TopPart(0.1));

        assert_eq!(0, indexes.len())
    }


    #[test]
    fn survive_top_one() {

        let mut world = World::new(Coord {x: 128, y: 128});
        let mut indiv = Individual::new();

        indiv.grid_index = 128*10;

        world.add_individual(indiv);

        let indexes = surviving_indexes(&world, SurvivalCriteria::TopPart(0.1));

        assert_eq!(1, indexes.len())
    }


}
