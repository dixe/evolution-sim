use sim_lib::*;

use failure;
use std::path::Path;
use gl_lib_sdl as gls;
use gl_lib_sdl::{
    gl_lib::text_rendering::font,
    gl_lib::na,
    sdl2
};

mod grid;
use grid::*;

mod live_text;
use live_text::*;


#[derive(Debug, Clone, Copy)]
pub enum Message {
    Run,
    Step,
    Pause,
    GridClick(Point),
    ReRender
}

fn main() -> Result<(), failure::Error> {

    let mut sim = sim_lib::simulation::SimulationBuilder::new(128, 128).build();

    sim.initialize_first_generation(None);


    let width = 1000;
    let height = 800;

    let font_path = Path::new("./assets/fonts/Arial.fnt");
    let font = font::Font::load_fnt_font(font_path).unwrap();
    let mut window = gls::window::SdlGlWindow::new("Evo Sim", width, height, font).unwrap();

    window.set_background_color(na::Vector4::new(0.9, 0.9, 0.9, 1.0));

    window.setup_blend();

    let mut model = Model { sim, run_state: RunState::Paused, cells: vec![], generation_text: LiveTextString { text: "Generation 0".to_string()}  };
    model.initialize_cells();

    while !window.should_quit() {

        if model.run_state == RunState::Running {
            run_single_generation(&mut model);
        }

        window.update(&mut model);
    }

    Ok(())
}


fn run_single_step(model: &mut Model) {
    model.sim.step_single_thread();
    model.update_cells();
    model.generation_text.text = format!("Generation {}", model.sim.generation());
}



fn run_single_generation(model: &mut Model) {
    model.sim.run_generation();

    model.update_cells();
    model.generation_text.text = format!("Generation {}", model.sim.generation());
}


#[derive(Debug, Clone, Copy, PartialEq)]
enum RunState {
    Running,
    Paused
}


#[derive(Clone)]
struct Model {
    sim: sim_lib::simulation::Simulation,
    run_state: RunState,
    cells: Vec::<Cell>,
    generation_text: LiveTextString
}

impl Model {
    pub fn initialize_cells(&mut self) {
        // initialize cells
        for indiv in &self.sim.world().individuals {
            let coord = index_functions::index_to_coord(indiv.grid_index, self.sim.world().grid.size);
            self.cells.push(Cell { cell_type: CellType::Square, color: Color::RGB(255,0,0), point: Point::new(coord.x, coord.y) })
        }
    }

    pub fn update_cells(&mut self) {

        for (i, indiv) in self.sim.world().individuals.iter().enumerate() {
            let coord = index_functions::index_to_coord(indiv.grid_index, self.sim.world().grid.size);
            self.cells[i].point = Point::new(coord.x, coord.y);
        }

    }

}



impl gls::State<Message> for Model {

    fn handle_message(&mut self, message: &Message, _window_access: &gls::window::WindowComponentAccess) {

        match message {
            Message::Step => {
                run_single_step(self);
            },
            Message::GridClick(point) => {
                println!("Grid clicked at: {:?}", point);
            }
            Message::Run => {
                self.run_state = RunState::Running
            },
            Message::Pause => {
                self.run_state = RunState::Paused
            },
            Message::ReRender => {},
        }
    }


    fn view(&self) -> gls::layout::Node<Message> {

        use gls::layout::*;

        use Length::*;

        let size = GridSize {rows: self.sim.world().grid.size.y, columns: self.sim.world().grid.size.x };

        let col = Column::new()
            .width(Fill)
            .add(top_row(&self))
            .add(Row::new()
                 .add(Button::new("", None)
                      .width(Px(20))
                 )
                 .add(GridLayout::new(size, &self.cells, Message::GridClick, Message::GridClick)
                      .width(Fill)
                      .max_width(600)
                      .max_height(600)
                      .height(Fill))
            );

        col.into()
    }
}


fn top_row(model: &Model) -> gls::layout::Row<Message> {
    use gls::layout::*;

    use Length::*;


    let mut row = Row::new()
        .padding(5.0)
        .spacing(10.0)
        .width(Fill);


    // left button

    row = row.add((match model.run_state {
        RunState::Running => {
            Button::new("Pause", Some(Message::Pause))

        },
        RunState::Paused => {
            Button::new("Run", Some(Message::Run))
        }
    }).height(Px(50)));

    row = row.add_if(model.run_state != RunState::Running, Button::new("Step", Some(Message::Step))
                     .height(Px(50)));


    row = row.add(LiveTextLayout::new(&model.generation_text, Some(Message::Step))
                  .height(Px(50))
                  .width(Px(400))
                  .align_center());

    row
}
