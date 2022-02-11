use gl_lib::na;


pub type Point = na::Vector2::<usize>;

#[derive(Debug, Clone, Copy)]
pub struct GridSize  {
    pub columns: usize,
    pub rows: usize,
}

#[derive(Debug, Clone)]
pub struct CellsInfo {
    pub cells: Vec<Vec::<Cell>>,
}

mod component;
pub use self::component::*;

mod layout_element;
pub use self::layout_element::*;
