use gl_lib::sdl_gui::components::base;
use gl_lib::{
        gl,
        objects::square,
        ScreenBox,
        text_rendering::{ text_renderer::{TextRenderer, TextAlignment} },
    };

use std::fmt;
use crate::live_text::*;


#[derive(Debug)]
pub struct LiveTextComponent<Message> {
    pub base: base::ComponentBase,
    left_clicked_message: Option::<Message>,
    text_pointer: *const LiveTextString,
    text_align: TextAlignment
}


impl<Message> LiveTextComponent<Message> where Message: Clone  {

    pub fn new(_gl: &gl::Gl, text_pointer: *const LiveTextString, text_align: TextAlignment, left_clicked_message: Option::<Message>) -> Box<Self> {
        Box::new(Self {
            base: Default::default(),
            left_clicked_message,
            text_pointer,
            text_align
        })
    }

    fn render_livetext(&self, gl: &gl::Gl, tr: &mut TextRenderer, screen_w: f32, screen_h: f32) {


        let screen_box = ScreenBox::new(self.base.x, self.base.y, self.base.width, self.base.height, screen_w, screen_h);

        let livetext;
        unsafe {
            livetext = &*self.text_pointer as &LiveTextString;
        }

        //println!("Live_text({}e = {:?}", &livetext.text[..10], self.base);


        tr.render_text(gl, &livetext.text, self.text_align, screen_box, livetext.scale);

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


    fn render(&self, gl: &gl::Gl, tr: &mut TextRenderer, _render_square: &square::Square, screen_w: f32, screen_h: f32) {
        self.render_livetext(gl, tr, screen_w, screen_h);
    }


    fn update_content(&mut self, _: String) {

    }

    fn on_event(&self, event: base::ComponentEvent) -> Option<Message> {
        if let Some(msg) = &self.left_clicked_message {
            return match event {
                base::ComponentEvent::Clicked(_, _ ) => {
                    Some(msg.clone())
                },
                _ => None
            }
        }

        None
    }
}
