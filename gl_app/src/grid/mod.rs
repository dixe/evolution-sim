use gl_lib_sdl::{
    gl_lib::na,
};


pub type Point = na::Vector2::<usize>;

#[derive(Debug, Clone, Copy)]
pub struct GridSize  {
    pub columns: usize,
    pub rows: usize,
}

mod component;
pub use self::component::*;

mod layout_element;
pub use self::layout_element::*;
