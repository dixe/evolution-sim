use crate::combined_types::*;
use crate::basic_types::*;
use crate::index_functions::*;


#[derive(Debug, Clone, Copy)]
pub enum SurvivalCriteria {
    TopPart(f32),
    BottomPart(f32),
    Border(f32),
    NoPheromones,
    RequirePheromones,
    Center(Coord, u32),
}


pub fn survive_cells(world: &World, criteria: SurvivalCriteria) -> Vec::<Coord> {

    let mut res = vec![];

    for x in 0..world.grid.size.x {
        for y in 0..world.grid.size.y {

            let coord = Coord {x, y};
            let grid_index = coord_to_index(coord, world.grid.size);

            if match_criteria(world, criteria, grid_index) {
                res.push(coord)
            }
        }
    }

    res

}


fn match_criteria(world: &World, criteria: SurvivalCriteria, grid_index: usize) -> bool {

    let coord = index_to_coord(grid_index, world.grid.size);

    match criteria {
        SurvivalCriteria::Center(center, radius) => in_center(world, grid_index, center, radius),
        SurvivalCriteria::NoPheromones => survive_no_pheromones(world, grid_index),
        SurvivalCriteria::TopPart(pct) => survive_top(world, pct, coord),
        SurvivalCriteria::Border(pct) => survive_border(world, pct, coord),
        SurvivalCriteria::BottomPart(pct) => survive_bottom(world, pct, coord),
        SurvivalCriteria::RequirePheromones => survive_pheromones(world, grid_index),
    }
}


pub fn surviving_indexes(world: &World, criteria: SurvivalCriteria) -> Vec::<usize> {
    let mut res = vec![];

    for indiv in &world.individuals {

        if match_criteria(world, criteria, indiv.grid_index){
            res.push(indiv.index);
        }
    }

    res
}


fn survive_no_pheromones(world: &World, grid_index: usize) -> bool {
    world.grid.tiles[grid_index].pheromone_level == 0
}

fn survive_pheromones(world: &World, grid_index: usize) -> bool {
    world.grid.tiles[grid_index].pheromone_level >= 10
}

fn survive_border(world: &World, pct: f32, coord: Coord) -> bool {

    let w = (world.grid.size.x as f32 * pct) as usize;
    let h = (world.grid.size.y as f32 * pct) as usize;

    coord.x <= w || coord.y <= h || coord.x >= world.grid.size.x - w || coord.y >= world.grid.size.y- h

}


fn in_center(world: &World, grid_index: usize , center: Coord, radius: u32) -> bool {

    let indiv_coord = index_to_coord(grid_index, world.grid.size);

    let x = indiv_coord.x as i32 - center.x as i32 ;
    let y = indiv_coord.y as i32 - center.y as i32 ;

    x * x + y*y < ((radius * radius) as i32)


}


fn survive_top(world: &World, pct: f32, coord: Coord) -> bool {

    let max_survive_y = (world.grid.size.y as f32 * pct) as usize;
    coord.y < max_survive_y
}


fn survive_bottom(world: &World, pct: f32, coord: Coord) -> bool {
    let min_survive_y = world.grid.size.y - (world.grid.size.y as f32 * pct) as usize;
    coord.y > min_survive_y
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

        assert_eq!(0, indexes.len());


        assert_eq!(false, survive_top(&world, 0.1, Coord {x: 128, y:128}));

        assert_eq!(false, survive_top(&world, 0.1, Coord {x: 0, y:128}));

        assert_eq!(false, survive_top(&world, 0.1, Coord {x: 1250, y:15}));


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


    #[test]
    fn survive_border_test() {

        let mut world = World::new(Coord {x: 128, y: 128});


        // TOP
        assert_eq!(true,survive_border(&world, 0.02, Coord {x: 123, y: 1} ));
        assert_eq!(true, survive_border(&world, 0.02, Coord {x: 123, y: 2} ));
        assert_eq!(false, survive_border(&world, 0.02, Coord {x: 123, y: 3} ));

        // BOTTOM
        assert_eq!(true, survive_border(&world, 0.02, Coord {x: 123, y: 127} ));
        assert_eq!(true, survive_border(&world, 0.02, Coord {x: 123, y: 126} ));
        assert_eq!(false, survive_border(&world, 0.02, Coord {x: 123, y: 125} ));

        assert_eq!(true, survive_border(&world, 0.02, Coord {x: 64, y: 126} ));


    }




}
