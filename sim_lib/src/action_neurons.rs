use crate::combined_types::*;
use crate::basic_types::*;


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
        EmitPheromone => {

        },
        SetOscPeriod => {}
        SetResponsivness => {},
    };
}





#[cfg(test)]
mod tests {

    use super::*;

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

}
