use sim_lib::*;

use failure;
use std::path::Path;
use gl_lib_sdl as gls;
use gl_lib_sdl::{
    gl_lib::text_rendering::font,
    gl_lib::na,
};

mod grid;
use grid::*;



#[derive(Debug, Clone, Copy)]
pub enum Message {
    Run,
    Step,
    Pause,
    GridClick(Point)
}

fn main() -> Result<(), failure::Error> {

    let mut sim = sim_lib::simulation::SimulationBuilder::new(128, 128).build();

    sim.initialize_first_generation(None);


    sim.step_single_thread();


    let width = 600;
    let height = 600;

    let font_path = Path::new("./assets/fonts/Arial.fnt");
    let font = font::Font::load_fnt_font(font_path).unwrap();
    let mut window = gls::window::SdlGlWindow::new("Evo Sim", width, height, font).unwrap();

    window.set_background_color(na::Vector4::new(0.9, 0.9, 0.9, 1.0));

    window.setup_blend();

    let mut state = State { sim };

    while !window.should_quit() {
        window.update(&mut state);
    }

    Ok(())

}



#[derive(Clone)]
struct State {
    sim: sim_lib::simulation::Simulation,
}

impl gls::State<Message> for State {

    fn handle_message(&mut self, message: &Message, _window_access: &gls::window::WindowComponentAccess) {

        match message {
            Message::Step => {

            },
            Message::GridClick(point) => {
                println!("Grid clicked at: {:?}", point);
            }
            _ => {},
        }

    }


    fn view(&self) -> gls::layout::Node<Message> {

        use gls::layout::*;

        use Length::*;

        let size = GridSize {rows: self.sim.world().grid.size.y, columns: self.sim.world().grid.size.x };

        let mut cells = vec![];
        for indiv in &self.sim.world().individuals {
            let coord = index_functions::index_to_coord(indiv.grid_index, self.sim.world().grid.size);
            cells.push(Cell { cell_type: CellType::Square, color: Color::RGB(255,0,0), cell: Point::new(coord.x, coord.y) })
        }


        let col = Column::new()
            .width(Fill)
            .add(Row::new()
                 .padding(5.0)
                 .width(Fill)
                 .add(Button::new("Start", Some(Message::Run))
                      .height(Px(50))
                 )
                 .add(Button::new("Pause", Some(Message::Pause))
                      .height(Px(50))
                      .align_center()
                 )
                 .add(Button::new("Pause", Some(Message::Step))
                      .height(Px(50))
                      .align_right()
                 ))
            .add(GridLayout::new(size, cells, Message::GridClick, Message::GridClick)
                 .width(Fill)
                 .height(Fill)
            );


        col.into()
    }
}
