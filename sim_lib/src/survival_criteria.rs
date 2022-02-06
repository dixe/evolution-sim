use crate::combined_types::*;
use crate::basic_types::*;
use crate::index_functions::*;


#[derive(Debug, Clone, Copy)]
pub enum SurvivalCriteria {
    TopPart(f32),
    BottomPart(f32),
    Border(f32)
}


pub fn survive_cells(world: &World, criteria: SurvivalCriteria) -> Vec::<Coord> {

    match criteria {
        SurvivalCriteria::TopPart(pct) => {

            let max_survive_y = (world.grid.size.y as f32 * pct) as usize;

            let mut res = vec![];
            for x in 0..world.grid.size.x {
                for y in 0..max_survive_y {
                    res.push(Coord{x,y});
                }
            }

            res
        },
        SurvivalCriteria::BottomPart(pct) => {

            let min_survive_y = world.grid.size.y - (world.grid.size.y as f32 * pct) as usize;

            let mut res = vec![];
            for x in 0..world.grid.size.x {
                for y in min_survive_y..world.grid.size.y {
                    res.push(Coord{x,y});
                }
            }

            res
        },
        SurvivalCriteria::Border(pct) => {

            let mut res = vec![];
            for x in 0..world.grid.size.x {
                for y in 0..world.grid.size.y {
                    let coord = Coord {x, y};

                    if survive_border(world, pct, coord) {
                        res.push(coord);
                    }
                }
            }

            res
        }
    }

}

pub fn surviving_indexes(world: &World, criteria: SurvivalCriteria) -> Vec::<usize> {
    let mut res = vec![];

    for indiv in &world.individuals {
        let coord = index_to_coord(indiv.grid_index, world.grid.size);

        if match criteria {
            SurvivalCriteria::TopPart(pct) => survive_top(world, pct, coord),
            SurvivalCriteria::Border(pct) => survive_border(world, pct, coord),
            SurvivalCriteria::BottomPart(pct) => survive_bottom(world, pct, coord),
        } {
            res.push(indiv.index);
        }
    }
    res
}


fn survive_border(world: &World, pct: f32, coord: Coord) -> bool {

    let w = (world.grid.size.x as f32 * pct) as usize;
    let h = (world.grid.size.y as f32 * pct) as usize;

    coord.x <= w || coord.y <= h || coord.x >= world.grid.size.x - w || coord.y >= world.grid.size.y- h

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


    }




}
