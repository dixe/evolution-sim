use crate::basic_types::*;


pub fn index_to_coord(grid_index: usize, size: Coord) -> Coord {
    Coord { x : (grid_index % size.x), y: grid_index / size.x }
}


pub fn coord_to_index(coord: Coord, size: Coord) -> usize {
    coord.y * size.x + coord.x

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_square() {

        let size = Coord {x: 10, y: 10};

        assert_eq!(Coord { x: 1, y: 1} , index_to_coord(11, size));
    }


    #[test]
    fn test_1() {

        let size = Coord {x: 128, y: 128};

        assert_eq!(Coord { x: 110, y: 64} , index_to_coord(8302, size));
    }




    #[test]
    fn test_rect() {

        let size = Coord {x: 20, y: 10};

        assert_eq!(Coord { x: 5, y: 1} , index_to_coord(25, size));
    }

    #[test]
    fn inverse() {

        let size = Coord {x: 128, y: 128};

        assert_eq!(431, coord_to_index(index_to_coord(431, size), size));

        assert_eq!(Coord { x: 65, y: 136}, index_to_coord(coord_to_index(Coord { x: 65, y: 136}, size), size));
    }

}
