use crate::combined_types::*;
use crate::basic_types::*;
use crate::index_functions;

pub fn perform_action(activation: &Activation, world: &mut World) {

    use Action::*;
    match activation.action {
        MoveForward => {

            // TODO: Thresholds. use indiv responsivness to modify
            if activation.weight < 0.2  {
                return;
            }

            let dir = world.individuals[activation.indiv_index].forward;

            world.move_indiv_dir(activation.indiv_index, dir);
        },
        MoveX => {

            // TODO: Thresholds. use indiv responsivness to modify
            if f64::abs(activation.weight) < 0.2  {
                return;
            }

            let dir = if activation.weight < 0.0 { Dir::Left } else { Dir::Right };

            world.move_indiv_dir(activation.indiv_index, dir);

        },
        MoveY => {
            // TODO: Thresholds. use indiv responsivness to modify
            if f64::abs(activation.weight) < 0.2  {
                return;
            }

            let dir = if activation.weight < 0.0 { Dir::Down } else { Dir::Up };

            world.move_indiv_dir(activation.indiv_index, dir);

        },
        EmitPheromones => { // pheromones in neighboor hood
            if activation.weight < 0.2  {
                return;
            }

            let radius : i32 = 5;

            let base_pheromone = 10.0;
            let center_grid_index = world.individuals[activation.indiv_index].grid_index;

            let center_coord = index_functions::index_to_coord(center_grid_index, world.grid.size);
            let radius_sqr = radius * radius;
            for x in i32::max(0, center_coord.x as i32 - radius)..=i32::min(center_coord.x as i32 + radius, (world.grid.size.x - 1) as i32) {
                for y in i32::max(0, center_coord.y as i32 - radius )..=i32::min(center_coord.y as i32 + radius,  (world.grid.size.y - 1) as i32) {
                    let dist = (x - center_coord.x as i32) * (x - center_coord.x as i32) + (y - center_coord.y as i32) * (y - center_coord.y as i32);

                    if dist <= radius_sqr {
                        let level = (- dist as f32 / 12.0).exp() * base_pheromone;
                        let grid_index = index_functions::coord_to_index(Coord{x: x as usize, y: y as usize }, world.grid.size);

                        world.grid.increment_pheromone(grid_index, level as u8);

                    }
                }
            }
        },
        SetOscPeriod => {}
        SetResponsivness => {},
    };
}





#[cfg(test)]
mod tests {

    use super::*;
    use crate::index_functions;
    fn create_test_world() -> World {
        let world = World::new(Coord {x: 128, y: 128});
        world
    }

    #[test]
    fn move_forward() {

        let mut world = create_test_world();

        let mut indiv = Individual::new();
        indiv.forward = Dir::Up;
        indiv.grid_index = 128/2 + 128* (128/2 -1);
        let old_grid_index = indiv.grid_index;
        let indiv_index = world.add_individual(indiv);



        perform_action(&Activation { action: Action::MoveForward, weight: 0.1, indiv_index}, &mut world);

        // No movement
        assert_eq!(old_grid_index, world.individuals[indiv_index].grid_index);

        perform_action(&Activation { action: Action::MoveForward, weight: 1.0, indiv_index }, &mut world);
        assert_eq!(old_grid_index - 128, world.individuals[indiv_index].grid_index);

    }

    #[test]
    fn move_down_last_row() {

        let mut world = create_test_world();

        let mut indiv = Individual::new();
        indiv.forward = Dir::Down;
        indiv.grid_index = 128*128 - 150;

        let old_grid_index = indiv.grid_index;
        let indiv_index = world.add_individual(indiv);

        let old_coord = index_functions::index_to_coord(old_grid_index, world.grid.size);


        assert_eq!(126, old_coord.y);


        perform_action(&Activation { action: Action::MoveForward, weight: 1.0, indiv_index}, &mut world);

        let new_coord = index_functions::index_to_coord(world.individuals[indiv_index].grid_index, world.grid.size);
        assert_eq!(127, new_coord.y);

    }


    #[test]
    fn emit_pheromones() {

        let mut world = create_test_world();

        let mut indiv = Individual::new();

        let index =  64*128 + 64;
        indiv.grid_index = index;

        let indiv_index = world.add_individual(indiv);

        perform_action(&Activation { action: Action::EmitPheromones, weight: 1.0, indiv_index}, &mut world);

        assert_eq!(10, world.grid.tiles[index].pheromone_level);

        assert_eq!(1, world.grid.tiles[index - 5].pheromone_level);

    }

     #[test]
    fn emit_pheromones_2() {

        let mut world = create_test_world();

        let mut indiv = Individual::new();

        let index =  0;
        indiv.grid_index = index;

        let indiv_index = world.add_individual(indiv);

        perform_action(&Activation { action: Action::EmitPheromones, weight: 1.0, indiv_index}, &mut world);

        assert_eq!(10, world.grid.tiles[index].pheromone_level);

        assert_eq!(1, world.grid.tiles[index + 128 * 5].pheromone_level);

    }


}
