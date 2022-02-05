use rand::Rng;
use crate::combined_types::*;
use crate::basic_types::*;



pub fn get_sensor_input(sensor: Sensor, world: &World, indiv: &Individual) -> f64 {


    match sensor {
        Sensor::Constant => 1.0,
        Sensor::Random => {
            let mut rng = rand::thread_rng();
            rng.gen_range(-1.0..1.0)
        },
        Sensor::LocX => location_x(world, indiv),
        Sensor::LocY => location_y(world, indiv),
        Sensor::WorldBorderDistX => world_border_dist_x(world, indiv),
        Sensor::WorldBorderDistY => world_border_dist_y(world, indiv),
        Sensor::BlockedForward => {
            if world.is_dir_empty(indiv.grid_index, indiv.forward) {
                0.0
            }
            else {
                1.0
            }
        },
    }
}




fn location_x(world: &World, indiv: &Individual) -> f64 {
    // given the index into world grid we can see how close to an edge we are by taking modolus width
    // multiply by 2 and subtact 2 to scale from 0..1 to -1..1 range
    let width_percentage = ((indiv.grid_index % world.grid.size.x) as f64) / world.grid.size.x as f64;
    width_percentage * 2.0 - 1.0
}

fn location_y(world: &World, indiv: &Individual) -> f64 {
    // given the index into world grid we can see how close to an edge we are by taking dividing by width
    let height_percentage = (indiv.grid_index / world.grid.size.y) as f64 / world.grid.size.y as f64;
    height_percentage * 2.0 - 1.0
}


// WOLRD BORDER_DIST_X
// 0 is at a border on X
// 1 in the middle
fn world_border_dist_x(world: &World, indiv: &Individual) -> f64 {
    let location = location_x(world, indiv);
    // given location x between -1 and 1 take the absolute value and subtract it from 1
    1.0 - f64::abs(location)
}
// WOLRD BORDER_DIST_Y
// 0 is at a border on Y
// 1 in the middle
fn world_border_dist_y(world: &World, indiv: &Individual) -> f64 {
    let location = location_y(world, indiv);
    // given location x between -1 and 1 take the absolute value and subtract it from 1
    1.0 - f64::abs(location)
}




#[cfg(test)]
mod tests {

    use super::*;
    use crate::basic_types::*;

    fn create_test_world() -> World {
        let world = World::new(Coord {x: 128, y: 128});
        world
    }

    #[test]
    fn location_test() {

        let world = create_test_world();

        let mut indiv_0_0 = Individual::new();
        indiv_0_0.grid_index= 128/2 + 128* 128/2;

        let dist_x = location_x(&world, &indiv_0_0);
        let dist_y = location_y(&world, &indiv_0_0);

        assert_eq!(0.0, dist_x);
        assert_eq!(0.0, dist_y);




        let mut indiv_bottom_west = Individual::new();
        indiv_bottom_west.grid_index= 128* 127;

        let dist_x = location_x(&world, &indiv_bottom_west);
        let dist_y = location_y(&world, &indiv_bottom_west);

        assert_relative_eq!(-1.0, dist_x, epsilon = 0.2);
        assert_relative_eq!(1.0, dist_y, epsilon = 0.2);


    }

    #[test]
    fn world_border_dist_test() {

        let world = create_test_world();

        let mut indiv_0_0 = Individual::new();
        indiv_0_0.grid_index= 128/2 + 128* 128/2;

        let dist_x = world_border_dist_x(&world, &indiv_0_0);
        let dist_y = world_border_dist_y(&world, &indiv_0_0);

        assert_eq!(1.0, dist_x);
        assert_eq!(1.0, dist_y);


        let mut indiv_bottom_west = Individual::new();
        indiv_bottom_west.grid_index= 128* 127;

        let dist_x = world_border_dist_x(&world, &indiv_bottom_west);
        let dist_y = world_border_dist_y(&world, &indiv_bottom_west);

        assert_relative_eq!(0.0, dist_x, epsilon = 0.2);
        assert_relative_eq!(0.0, dist_y, epsilon = 0.2);
    }

    #[test]
    fn blocked_forward_test() {

        let mut world = create_test_world();

        //
        let mut indiv_64_64 = Individual::new();
        indiv_64_64.forward = Dir::Up;
        indiv_64_64.grid_index = 64 + 128* 128/2;
        let i0_index = world.add_individual(indiv_64_64);


        let mut indiv_64_63 = Individual::new();
        indiv_64_63.forward = Dir::Up;
        indiv_64_63.grid_index = 128/2 + 128* (128/2 -1);
        let i1_index = world.add_individual(indiv_64_63);


        let is_blocked = get_sensor_input(Sensor::BlockedForward, &world, &world.individuals[i0_index]);

        assert_eq!(1.0, is_blocked);

        let is_blocked = get_sensor_input(Sensor::BlockedForward, &world, &world.individuals[i1_index]);
        assert_eq!(0.0, is_blocked);

        //TODO: Maybe also test Down Left and Right

    }


}
