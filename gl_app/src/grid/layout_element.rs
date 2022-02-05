use gl_lib_sdl::layout::*;
use gl_lib_sdl::{
    gl_lib::{
        gl,
        text_rendering::{ text_renderer::TextRenderer },
    }
};
use gl_lib_sdl::components::base::*;
use std::fmt;

use crate::grid::*;

#[derive(Debug)]
pub struct GridLayout<Message> {
    attributes: Attributes,
    left_clicked_message: fn(Point) -> Message,
    right_clicked_message: fn(Point) -> Message,
    size: GridSize,
    cells_info: *const CellsInfo, // Use a pointer for now, to make easy realtime update
}


impl<Message> GridLayout<Message> where Message: Clone {
    pub fn new(size: GridSize, cells_info: *const CellsInfo, left_clicked_message: fn(Point) -> Message, right_clicked_message: fn(Point) -> Message) -> Self {
        Self {
            attributes: Default::default(),
            left_clicked_message,
            right_clicked_message,
            size,
            cells_info,
        }
    }
}

impl<Message> Element<Message> for GridLayout<Message> where Message: 'static + Clone + fmt::Debug {

    fn name(&self) -> String {
        "GridLayout".to_string()
    }

    fn attributes(&self) -> &Attributes {
        &self.attributes
    }

    fn attributes_mut(&mut self) -> &mut Attributes {
        &mut self.attributes
    }

    fn content_height(&self, available_space: &RealizedSize, _text_renderer: &TextRenderer) -> f32 {
        available_space.height
    }

    fn content_width(&self, available_space: &RealizedSize, _text_renderer: &TextRenderer) -> f32 {
        available_space.width
    }

    fn create_component(&self, gl: &gl::Gl, comp_base: ComponentBase) -> Option<Component<Message>> {
        let mut grid: Component<Message> = GridComponent::new(gl, self.size, self.cells_info, self.left_clicked_message, self.right_clicked_message);
        grid.set_base(comp_base);
        Some(grid)
    }

    fn pop_children_front(&mut self) -> Option<Node<Message>> where Message: fmt::Debug {
        None
    }

}


impl<Message: 'static> From<GridLayout<Message>> for Node<Message>
where
    Message: Clone + fmt::Debug   {

    fn from(grid: GridLayout<Message>) -> Node<Message> {
        Box::new(grid)
    }

}
