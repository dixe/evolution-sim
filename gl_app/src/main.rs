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

mod live_text;
use live_text::*;


#[derive(Debug, Clone, Copy)]
pub enum Message {
    Evolve,
    RunSingle,
    Step,
    StepGen,
    Pause,
    GridClick(Point),
    ReRender
}

static INDIV_CELL_INDEX : usize = 0;
static SURVIVE_CELL_INDEX : usize = 1;
static PHEROMONE_CELL_INDEX : usize = 2;


fn main() -> Result<(), failure::Error> {

    let mut sim =
        sim_lib::simulation::SimulationBuilder::new(128, 128)
        .genome_length(10)
        .mutation_rate(0.1)
        .build();

    sim.initialize_first_generation(None);


    let width = 1000;
    let height = 800;

    let font_path = Path::new("./assets/fonts/Arial.fnt");
    let font = font::Font::load_fnt_font(font_path).unwrap();
    let mut window = gls::window::SdlGlWindow::new("Evo Sim", width, height, font).unwrap();

    window.set_background_color(na::Vector4::new(0.9, 0.9, 0.9, 1.0));

    window.setup_blend();

    let mut model =
        Model {
            sim,
            run_state: RunState::Paused,
            cells_info: CellsInfo { cells: vec![vec![], vec![], vec![]] },
            generation_text: LiveTextString { text: "Generation 0".to_string(), scale: 1.0 },
            stat_text: LiveTextString { text: "".to_string(), scale: 0.8 }
        };


    model.initialize_cells();



    while !window.should_quit() {

        match model.run_state {
            RunState::Evolving => {
                run_single_generation(&mut model);
            },
            RunState::RunSingleGen => {
                let last_step = run_single_step(&mut model);

                if last_step {
                    model.update_stats();
                    window.send_message(Message::Pause);
                }
            },
            RunState::Paused => { }
        };


        window.update(&mut model);
    }

    Ok(())
}


fn run_single_step(model: &mut Model) -> bool {
    let last_step = model.sim.step_single_thread();
    model.update_cells();
    model.generation_text.text = format!("Generation {}", model.sim.generation());
    last_step
}



fn run_single_generation(model: &mut Model) {
    model.sim.run_generation();

    model.update_cells();
    model.generation_text.text = format!("Generation {}", model.sim.generation());
}


#[derive(Debug, Clone, Copy, PartialEq)]
enum RunState {
    Evolving,
    RunSingleGen,
    Paused
}


#[derive(Clone)]
struct Model {
    sim: sim_lib::simulation::Simulation,
    run_state: RunState,
    cells_info: CellsInfo,
    generation_text: LiveTextString,
    stat_text: LiveTextString,
}

impl Model {
    pub fn initialize_cells(&mut self) {
        // individuals cells
        for indiv in &self.sim.world().individuals {
            let coord = index_functions::index_to_coord(indiv.grid_index, self.sim.world().grid.size);
            let color = gene_functions::genome_to_rgb(&indiv.genome);
            self.cells_info.cells[INDIV_CELL_INDEX].push(Cell { cell_type: CellType::Square, color: Color::RGB(color.0, color.1, color.2), point: Point::new(coord.x, coord.y) })
        }

        // survive cells
        for coord in &self.sim.survive_cells() {
            self.cells_info.cells[SURVIVE_CELL_INDEX].push(Cell { cell_type: CellType::Square, color: Color::RGBA(53, 212, 63, 50), point: Point::new(coord.x, coord.y) })
        }

        // pheromones



    }

    pub fn update_cells(&mut self) {

        for (i, indiv) in self.sim.world().individuals.iter().enumerate() {
            let coord = index_functions::index_to_coord(indiv.grid_index, self.sim.world().grid.size);
            self.cells_info.cells[0][i].point = Point::new(coord.x, coord.y);


            let color = gene_functions::genome_to_rgb(&indiv.genome);
            self.cells_info.cells[0][i].color = Color::RGB(color.0, color.1, color.2);
        }
    }

    pub fn update_stats(&mut self) {
        let config = self.sim.config();

        self.stat_text.text = format!("{:#?}\nSurvival rate: {:.0} %", config, self.sim.last_survival_rate());
    }
}



impl gls::State<Message> for Model {

    fn handle_message(&mut self, message: &Message, _window_access: &gls::window::WindowComponentAccess) {

        self.update_stats();

        match message {
            Message::Step => {
                run_single_step(self);
            },
            Message::StepGen => {
                run_single_generation(self);
            },
            Message::GridClick(point) => {
                println!("Grid clicked at: {:?}", point);
            }
            Message::RunSingle => {
                self.run_state = RunState::RunSingleGen;
            },
            Message::Evolve => {
                self.run_state = RunState::Evolving
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
            .padding(20.0)
            .add(top_row(&self))
            .add(Row::new()
                 .add(GridLayout::new(size, &self.cells_info, Message::GridClick, Message::GridClick)
                      .width(Px(600))
                      .max_width(600)
                      .max_height(600)
                      .height(Fill)
                 )
                 .add(LiveTextLayout::new(&self.stat_text, None)
                      .height(Px(600))
                      .width(Fill)
                      .align_bottom())

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

    if model.run_state == RunState::Paused {
        row = row
            .add(Button::new("Evolve", Some(Message::Evolve))
                 .height(Px(50)))
            .add(Button::new("Run", Some(Message::RunSingle))
                 .height(Px(50)))
            .add(Button::new("Step", Some(Message::Step))
                 .height(Px(50)))

            .add(Button::new("Step Generation", Some(Message::StepGen))
                 .height(Px(50)))
    }
    else {
        row = row.add(Button::new("Pause", Some(Message::Pause))
                      .height(Px(50)));
    }


    row = row.add(LiveTextLayout::new(&model.generation_text, None)
                  .height(Px(50))
                  .width(Px(400))
                  .align_center());

    row
}
