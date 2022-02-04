use crate::basic_types::*;


pub fn index_to_coord(grid_index: usize, size: Coord) -> Coord {
    Coord { x : (grid_index % size.x), y: grid_index / size.x }
}



#[cfg(test)]
mod tests {

    use super::*;
    use crate::basic_types::*;

    #[test]
    fn test_square() {

        let size = Coord {x: 10, y: 10};

        assert_eq!(Coord { x: 1, y: 1} , index_to_coord(11, size));
    }




    #[test]
    fn test_rect() {

        let size = Coord {x: 20, y: 10};

        assert_eq!(Coord { x: 5, y: 1} , index_to_coord(25, size));
    }


}
