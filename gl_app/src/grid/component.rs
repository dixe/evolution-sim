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
use crate::grid::*;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    RGB(u8, u8, u8),
    RGBA(u8, u8, u8, u8)
}

impl Color {

    pub fn to_gl_color(&self) -> na::Vector4::<f32> {
        match self {
            Color::RGB(r, g, b) => {
                na::Vector4::new(*r as f32 / 255.0, *g as f32 / 255.0, *b as f32 / 255.0, 1.0)
            }
            Color::RGBA(r, g, b, a) => {
                na::Vector4::new(*r as f32 / 255.0, *g as f32 / 255.0, *b as f32 / 255.0, *a as f32 / 255.0)
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub cell_type: CellType,
    pub color: Color,
    pub point: Point
}


#[derive(Debug, Clone, Copy)]
pub enum CellType {
    Square,
    //Text(String)
}

#[derive(Debug)]
pub struct GridComponent<Message> {
    pub grid_shader: Shader,
    pub cell_shader: Shader,
    pub base: base::ComponentBase,
    size: GridSize,
    left_clicked_message: fn(Point) -> Message,
    right_clicked_message: fn(Point) -> Message,
    cells_info: *const CellsInfo
}


impl<Message> GridComponent<Message> where Message: Clone  {

    pub fn new(gl: &gl::Gl, size: GridSize, cells_info: *const CellsInfo, left_clicked_message: fn(Point) -> Message, right_clicked_message: fn(Point) -> Message) -> Box<Self> {
        let grid_shader = grid_shader(gl).unwrap();

        let cell_shader = cell_shader(gl).unwrap();

        Box::new(Self {
            grid_shader,
            cell_shader,
            size: size,
            base: Default::default(),
            left_clicked_message,
            right_clicked_message,
            cells_info
        })
    }

    fn render_grid(&self, gl: &gl::Gl, transform: na::Matrix4::<f32>, render_square: &square::Square) {


        self.grid_shader.set_used();

        self.grid_shader.set_mat4(gl, "transform", transform);

        self.grid_shader.set_f32(gl, "height", self.base.height );

        self.grid_shader.set_f32(gl, "width", self.base.width);

        self.grid_shader.set_vec2(gl, "grid_size", na::Vector2::new(self.size.columns as f32, self.size.rows as f32));

        render_square.render(&gl);
    }

    fn render_cells(&self, gl: &gl::Gl, render_square: &square::Square, screen_w: f32, screen_h: f32) {

        let info;
        unsafe {
            info =  &*self.cells_info as &CellsInfo;
        }

        for cells in &info.cells {
            for cell in cells {
                //let transform = self.base.unit_square_transform_matrix(screen_w as f32, screen_h as f32);
                let transform = self.cell_transform_matrix(cell.point, screen_w, screen_h);
                self.cell_shader.set_used();

                self.cell_shader.set_mat4(gl, "transform", transform);

                self.cell_shader.set_f32(gl, "height", self.base.height / self.size.rows as f32);

                self.cell_shader.set_f32(gl, "width", self.base.width / self.size.columns as f32);

                self.cell_shader.set_vec4(gl, "u_color", cell.color.to_gl_color());

                render_square.render(&gl);
            }
        }

    }

    pub fn cell_transform_matrix(&self, point: Point, screen_w: f32, screen_h: f32) -> na::Matrix4::<f32> {

        // TODO: move to top of render_cells and only do math ones, except that which depends on point
        // TODO: maybe points in as a buffer and render all "at the same time"

        // Get screen coord top left and bottom right. Out screen space is inside this rect
        let mut sc_top_left = base::ComponentBase::window_to_screen_coords(self.base.x, self.base.y, screen_w, screen_h);
        let mut sc_bottom_right = base::ComponentBase::window_to_screen_coords(self.base.x + self.base.width, self.base.y + self.base.height , screen_w, screen_h);


        // remove border space from cell space
        // TODO: do this more correct, so it scales in size well
        sc_top_left.x += 0.01;
        sc_top_left.y -= 0.01;


        let sc_width = sc_bottom_right.x - sc_top_left.x;
        let sc_height = sc_bottom_right.y - sc_top_left.y;

        let screen_x_scale = self.base.width  / screen_w  * 2.0;
        let screen_y_scale = self.base.height / screen_h * 2.0;

        let x_scale = screen_x_scale * (1.0 / self.size.columns as f32);
        let y_scale = screen_y_scale * (1.0 / self.size.rows as f32);
        let mut model = na::Matrix4::<f32>::identity();

        // Scale to size
        model[0] = x_scale;
        model[5] = y_scale;

        // Move first to begining of our screen space
        let mut x_move = sc_top_left.x + point.x as f32 / self.size.columns as f32 * sc_width;
        let mut y_move = sc_top_left.y + point.y as f32 / self.size.rows as f32 * sc_height;


        let trans = Translation3::new(x_move, y_move, 0.0);


        model = trans.to_homogeneous() * model;

        model

    }


}


impl<Message> base::ComponentTrait<Message> for GridComponent<Message> where Message: Clone + fmt::Debug {

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

        let grid_transform = self.base.unit_square_transform_matrix(screen_w as f32, screen_h as f32);

        self.render_grid(gl, grid_transform, render_square);

        self.render_cells(gl, render_square, screen_w, screen_h);

        /*

        self.render_flagged(gl, tr, screen_w, screen_h);

        self.render_suggestion(gl, render_square, screen_w, screen_h);

        if self.game_info.died {
        self.render_bombs(gl, tr, screen_w, screen_h);
    }

         */
    }

    fn update_content(&mut self, _: String) {

    }

    fn on_event(&self, event: base::ComponentEvent) -> Option<Message> {
        match event {
            base::ComponentEvent::Clicked(click_type, vec2) => {

                let offset = na::Vector2::new(self.base.x as i32, self.base.y as i32);
                let relative = vec2 - offset;

                let x = ((relative.y as f32 / self.base.height ) * self.size.rows as f32) as usize;
                let y = ((relative.x as f32 / self.base.width ) * self.size.columns as f32) as usize;

                match click_type {
                    base::ClickType::Left => Some((self.left_clicked_message)(Point::new(x,y))),
                    base::ClickType::Right => Some((self.right_clicked_message)(Point::new(x,y)))
                }
            },
            _ => None
        }
    }
}



/// Creates a shader for rendering a grid on a square (two triangle)
pub fn grid_shader(gl: &gl::Gl) -> Result<Shader, failure::Error> {

    // default program for square
    let vert_source = std::include_str!("grid_shader.vert");
    let frag_source = std::include_str!("grid_shader.frag");

    Shader::new(gl, vert_source, frag_source)
}



/// Creates a shader for rendering a grid on a square (two triangle)
pub fn cell_shader(gl: &gl::Gl) -> Result<Shader, failure::Error> {

    // default program for square
    let vert_source = std::include_str!("cell_shader.vert");
    let frag_source = std::include_str!("cell_shader.frag");

    Shader::new(gl, vert_source, frag_source)
}
