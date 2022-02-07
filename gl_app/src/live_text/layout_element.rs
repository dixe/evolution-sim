use gl_lib_sdl::layout::*;
use gl_lib_sdl::{
    gl_lib::{
        gl,
        text_rendering::text_renderer::{TextRenderer, TextAlignment, TextAlignmentX, TextAlignmentY},
    }
};
use gl_lib_sdl::components::base::*;
use std::fmt;

use crate::live_text::*;

#[derive(Debug)]
pub struct LiveTextLayout<Message> {
    attributes: Attributes,
    text_pointer: *const LiveTextString,
    left_clicked_message: Option<Message>
}


impl<Message> LiveTextLayout<Message> where Message: Clone {
    pub fn new(text: &LiveTextString, left_clicked_message: Option::<Message>) -> Self {

        let text_pointer = text as *const LiveTextString;

        Self {
            attributes: Default::default(),
            left_clicked_message,
            text_pointer,
        }
    }
}

impl<Message> Element<Message> for LiveTextLayout<Message> where Message: 'static + Clone + fmt::Debug {

    fn name(&self) -> String {
        "LiveTextLayout".to_string()
    }

    fn attributes(&self) -> &Attributes {
        &self.attributes
    }

    fn attributes_mut(&mut self) -> &mut Attributes {
        &mut self.attributes
    }

    fn content_height(&self, available_space: &RealizedSize, text_renderer: &TextRenderer) -> f32 {
        let max_width = self.contrainted_width(available_space);
        let text;
        unsafe {
            text = &(&*self.text_pointer as &LiveTextString).text;
        }

        text_renderer.render_box(text, max_width, 1.0).total_height
    }

    fn content_width(&self, available_space: &RealizedSize, text_renderer: &TextRenderer) -> f32 {
        let max_width = self.contrainted_width(available_space);
        let text;
        unsafe {
            text = &(&*self.text_pointer as &LiveTextString).text;
        }

        text_renderer.render_box(text, max_width, 1.0).total_width
    }

    fn create_component(&self, gl: &gl::Gl, comp_base: ComponentBase) -> Option<Component<Message>> {
        let mut alignment : TextAlignment = Default::default();

        // TODO: Move this into more general place, since it might be used by other elements when moved to gl-lib-sdl
        // TODO: Or maybe just only have 1 alignment that lives in rust-gl-lib
        alignment.x = match self.attributes.align.x {
            AlignmentX::Left => TextAlignmentX::Left,
            AlignmentX::Right => TextAlignmentX::Right,
            AlignmentX::Center =>TextAlignmentX::Center,
        };

        alignment.y = match self.attributes.align.y {
            AlignmentY::Top => TextAlignmentY::Top,
            AlignmentY::Bottom => TextAlignmentY::Bottom,
            AlignmentY::Center =>TextAlignmentY::Center,
        };

        let mut live_text: Component<Message> = LiveTextComponent::new(gl, self.text_pointer, alignment, self.left_clicked_message.clone());
        live_text.set_base(comp_base);
        Some(live_text)
    }

    fn pop_children_front(&mut self) -> Option<Node<Message>> where Message: fmt::Debug {
        None
    }

}


impl<Message: 'static> From<LiveTextLayout<Message>> for Node<Message>
where
    Message: Clone + fmt::Debug   {

    fn from(live_text: LiveTextLayout<Message>) -> Node<Message> {
        Box::new(live_text)
    }
}
