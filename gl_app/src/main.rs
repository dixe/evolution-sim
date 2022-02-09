use sim_lib::*;

use sim_lib::survival_criteria as sc;

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
    ReRender,
    ShowSurvivers,

}

static INDIV_CELL_INDEX : usize = 0;
static SURVIVE_CELL_INDEX : usize = 1;
static PHEROMONE_CELL_INDEX : usize = 2;


fn main() -> Result<(), failure::Error> {

    let mut sim =
        sim_lib::simulation::SimulationBuilder::new(128, 128)
        .genome_length(10)
        .criteria(sc::SurvivalCriteria::PheromoneInterval(20, 100))
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


        // INITIAL NO PHEROMONES

    }

    pub fn update_cells(&mut self) {

        for (i, indiv) in self.sim.world().individuals.iter().enumerate() {
            let coord = index_functions::index_to_coord(indiv.grid_index, self.sim.world().grid.size);
            self.cells_info.cells[INDIV_CELL_INDEX][i].point = Point::new(coord.x, coord.y);


            let color = gene_functions::genome_to_rgb(&indiv.genome);
            self.cells_info.cells[INDIV_CELL_INDEX][i].color = Color::RGB(color.0, color.1, color.2);
        }

        // clear old pheromones
        self.cells_info.cells[PHEROMONE_CELL_INDEX].clear();

        for (i, tile) in self.sim.world().grid.tiles.iter().enumerate() {

            if tile.pheromone_level > 0 {

                // scale to between 0 and 100
                let alpha = 15.0 + tile.pheromone_level as f32 / 3.0;

                let coord = index_functions::index_to_coord(i, self.sim.world().grid.size);


                self.cells_info.cells[PHEROMONE_CELL_INDEX].push(
                    Cell {
                        cell_type: CellType::Square,
                        color: Color::RGBA(125, 65, 204, alpha as u8),
                        point: Point::new(coord.x, coord.y) });
            }
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
            Message::ShowSurvivers => {

                // Set all to hidden with alpha = 0
                for i in 0..self.sim.world().individuals.len() {
                    self.cells_info.cells[INDIV_CELL_INDEX][i].color = Color::RGBA(0, 0, 0, 0);
                }

                let indivs = &self.sim.world().individuals;

                for i in self.sim.surviving_indexes() {
                    let color = gene_functions::genome_to_rgb(&indivs[i].genome);
                    self.cells_info.cells[INDIV_CELL_INDEX][i].color = Color::RGB(color.0, color.1, color.2);
                }
            },
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
                 .add(Column::new()
                      .add(LiveTextLayout::new(&self.generation_text, None)
                           .padding_bottom(10)
                           .width(Fill))
                      .add(LiveTextLayout::new(&self.stat_text, None)
                           .width(Fill)
                      ))

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
            .add(Button::new("Show Surviver", Some(Message::ShowSurvivers))
                 .height(Px(50))
                 .align_right())
    }
    else {
        row = row.add(Button::new("Pause", Some(Message::Pause))
                      .height(Px(50)));
    }

    row
}
