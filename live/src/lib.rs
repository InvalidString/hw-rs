pub mod graphics;

use graphics::*;

mod logic;

pub struct State<'g>{
    g: &'g mut Graphics,
    sdf_shader: Shader,
    font: SdfFont,
    model: Model,
}

pub struct Ui<'a, 'b, 'c>{
    g: &'c mut DrawHandle<'a>,
    font: SdfFont,
    sdf_shader: &'b Shader,
    text_scale: f32,
}
impl<'a, 'b, 'c> Ui<'a, 'b, 'c>{
    pub fn draw_text(&mut self, text: &str, pos: Vec2, color: Color){
        //self.g.g.draw_text(&self.font, text, pos, self.text_scale, color);
        self.g.draw_text_shader(self.font, self.sdf_shader, text, pos, self.text_scale, color)
    }
}
impl<'a, 'b, 'c> std::ops::Deref for Ui<'a, 'b, 'c> {
    type Target = DrawHandle<'a>;

    fn deref(&self) -> &Self::Target {
        &self.g
    }
}
impl<'a, 'b, 'c> std::ops::DerefMut for Ui<'a, 'b, 'c> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.g
    }
}

pub fn init(g: &mut Graphics)->State{

    let font_data = FileData::load("DejaVuSansMono.ttf").unwrap();
        
    let font = g.font_from_file_sdf(&font_data, 100);

    let shader = g.load_shader(None, "sdf.fs");

    let model = init_model(&g);
    State{
        g,
        sdf_shader: shader,
        font,
        model,
    }
}

#[derive(Debug)]
struct Model{
}

fn init_model(g: &Graphics)->Model{
    Model {
    }
}


#[no_mangle]
pub fn update(state: &mut State){
    let text_scale = 15.0;

    state.g.draw_frame(|g|{
        g.clear_background(colors::BLACK);
        let mut ui = Ui{
            g,
            font: state.font,
            sdf_shader: &state.sdf_shader,
            text_scale,
        };
        ui.draw_fps(10, 10);

    });
}


#[no_mangle]
pub fn should_close(state: &mut State)->bool{
    state.g.window_should_close()
}

#[no_mangle]
pub fn should_reload(state: &mut State)->bool{
    state.g.is_key_pressed(Key::R)
}
