use gl_lib_sdl::{
    components::base,
    gl_lib::{
        gl,
        na,
        na::Translation3,
        objects::square,
        ScreenBox,
        shader::Shader,
        text_rendering::{ text_renderer::{TextRenderer, TextAlignment, TextAlignmentX, TextAlignmentY} },
    }
};
use std::fmt;
use crate::live_text::*;


#[derive(Debug)]
pub struct LiveTextComponent<Message> {
    pub base: base::ComponentBase,
    left_clicked_message: Option::<Message>,
    text_pointer: *const LiveTextString
}


impl<Message> LiveTextComponent<Message> where Message: Clone  {

    pub fn new(gl: &gl::Gl, text_pointer: *const LiveTextString, left_clicked_message: Option::<Message>) -> Box<Self> {
        Box::new(Self {
            base: Default::default(),
            left_clicked_message,
            text_pointer
        })
    }

    fn render_livetext(&self, gl: &gl::Gl, tr: &mut TextRenderer, render_square: &square::Square, screen_w: f32, screen_h: f32) {

        let screen_box = ScreenBox::new(self.base.x, self.base.y, self.base.width, self.base.height, screen_w, screen_h);

        let livetext;
        unsafe {
            livetext = &*self.text_pointer as &LiveTextString;
        }

        tr.render_text(gl, &livetext.text, Default::default(), screen_box, livetext.scale);



    }
}


impl<Message> base::ComponentTrait<Message> for LiveTextComponent<Message> where Message: Clone + fmt::Debug {

    fn base(&self) -> &base::ComponentBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut base::ComponentBase {
        &mut self.base
    }

    fn set_base(&mut self, base: base::ComponentBase) {
        self.base = base;
    }


    fn render(&self, gl: &gl::Gl, tr: &mut TextRenderer, render_square: &square::Square, screen_w: f32, screen_h: f32) {
        self.render_livetext(gl, tr, render_square, screen_w, screen_h);
    }


    fn update_content(&mut self, _: String) {

    }

    fn on_event(&self, event: base::ComponentEvent) -> Option<Message> {
        if let Some(msg) = &self.left_clicked_message {
            return match event {
                base::ComponentEvent::Clicked(click_type, _ ) => {
                    Some(msg.clone())
                },
                _ => None
            }
        }

        None
    }
}
