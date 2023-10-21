use std::borrow::Borrow;
use std::ops::Deref;


mod rect;
pub use rect::*;

use raylib_ffi as ffi;
use ffi::enums::*;
pub use ffi::enums::KeyboardKey as Key;
pub use ffi::enums::MouseButton as MouseButton;
pub use ffi::enums::ConfigFlags;

pub use ffi::Color;
pub use raylib_ffi::colors;

pub type Pos2 = Vec2;

pub struct Graphics(());
pub struct DrawHandle<'a>{
    pub g: &'a mut Graphics,
}
impl<'a> Deref for DrawHandle<'a> {
    type Target = &'a mut Graphics;

    fn deref(&self) -> &Self::Target {
        &self.g
    }
}
impl<'a> DrawHandle<'a> {
    pub fn time(&self)->f64{
        unsafe{
            ffi::GetTime()
        }
    }
    pub fn mouse_pos(&self)->Pos2{
        unsafe{
            ffi::GetMousePosition()
        }.into()
    }
    pub fn mouse_delta(&self)->Vec2{
        unsafe{
            ffi::GetMouseDelta()
        }.into()
    }
    pub fn clear_background(&mut self, color: Color){
        unsafe{
            ffi::ClearBackground(color)
        }
    }
    pub fn draw_fps(&mut self, pos_x: i32, pos_y: i32){
        unsafe{
            ffi::DrawFPS(pos_x, pos_y)
        }
    }
    pub fn draw_circle(&mut self, center: Pos2, radius: f32, color: Color){
        unsafe{
            ffi::DrawCircleV(center.into(), radius, color)
        }
    }
    pub fn draw_circle_lines(&mut self, center: Pos2, radius: f32, color: Color){
        unsafe{
            ffi::DrawRingLines(center.into(), radius, radius, 0.0, 360.0, 36, color);
        }
    }
    pub fn draw_text(&mut self, text: &str, pos_x: i32, pos_y: i32, font_size: i32, color: Color){
        unsafe{
            ffi::DrawText(ffi::rl_str!(text), pos_x, pos_y, font_size, color)
        }
    }
    pub fn draw_text_shader(&mut self, font: SdfFont, shader: &Shader,text: &str, pos: Vec2, scale: f32, color: Color){
        unsafe{
            ffi::BeginShaderMode(shader.0);
            ffi::DrawTextEx(font.font, ffi::rl_str!(text), pos.into(), scale, 0.0, color);
            ffi::EndShaderMode();
        }
    }
    pub fn draw_rect_rounded_lines(&mut self, rect: Rect, roundness: f32, segments: i32, line_thickness: f32, color: Color){
        unsafe{
            ffi::DrawRectangleRoundedLines(rect.into(), roundness, segments, line_thickness, color)
        }
    }
    pub fn draw_rect_lines(&mut self, rect: Rect, line_thickness: f32, color: Color){
        unsafe{
            ffi::DrawRectangleRoundedLines(rect.into(), 0.0, 0, line_thickness, color)
        }
    }
    pub fn draw_rect_rounded(&mut self, rect: Rect, roundness: f32, segments: i32, color: Color){
        unsafe{
            ffi::DrawRectangleRounded(rect.into(), roundness, segments, color)
        }
    }
    pub fn draw_rect(&mut self, rect: Rect, color: Color){
        unsafe{
            ffi::DrawRectangleRec(rect.into(), color)
        }
    }

}

impl Graphics{
    pub fn is_window_resized(&self)->bool{
        unsafe{
            ffi::IsWindowResized()
        }
    }
    pub fn window_size(&self)->Vec2{
        let (x,y) = unsafe{
            (
                ffi::GetScreenWidth(),
                ffi::GetScreenHeight(),
            )
        };
        vec2(x as f32, y as f32)
    }
    pub fn window_should_close(&mut self)->bool{
        unsafe{
            ffi::WindowShouldClose()
        }
    }
    pub fn init_ex(width: i32, height: i32, title: &str, flags: &[ConfigFlags])->Self{
        let mask = flags.iter().map(|x|*x as u32).fold(0, |x,y| x|y);
        unsafe{
            ffi::SetConfigFlags(mask);
            ffi::InitWindow(width, height, ffi::rl_str!(title));
            ffi::SetTargetFPS(60);
        }
        Graphics(())
    }
    pub fn init(width: i32, height: i32, title: &str)->Self{
        Self::init_ex(width, height, title, &[ConfigFlags::Msaa4xHint])
    }
    pub fn draw_frame<F: FnMut(&mut DrawHandle)>(&mut self, mut f: F){
        unsafe{
            ffi::BeginDrawing();
        }
        f(&mut DrawHandle{
            g: self
        });
        unsafe{
            ffi::EndDrawing();
        }
    }
    pub fn is_key_pressed(&self, key: ffi::enums::KeyboardKey)->bool{
        unsafe{
            ffi::IsKeyPressed(key as i32)
        }
    }
    pub fn is_key_down(&self, key: ffi::enums::KeyboardKey)->bool{
        unsafe{
            ffi::IsKeyDown(key as i32)
        }
    }
    pub fn is_mouse_button_pressed(&self, button: ffi::enums::MouseButton)->bool{
        unsafe{
            ffi::IsMouseButtonPressed(button as i32)
        }
    }
    pub fn is_mouse_button_released(&self, button: ffi::enums::MouseButton)->bool{
        unsafe{
            ffi::IsMouseButtonReleased(button as i32)
        }
    }
    pub fn is_mouse_button_down(&self, button: ffi::enums::MouseButton)->bool{
        unsafe{
            ffi::IsMouseButtonDown(button as i32)
        }
    }
    pub fn is_mouse_button_up(&self, button: ffi::enums::MouseButton)->bool{
        unsafe{
            ffi::IsMouseButtonUp(button as i32)
        }
    }
}




impl Graphics{
    pub fn chars_pressed(&self) -> CharsPressed{
        CharsPressed(())
    }
}
pub struct CharsPressed(());
impl Iterator for CharsPressed{
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match unsafe{ffi::GetCharPressed()}{
            0 => None,
            x => char::from_u32(x as u32)
        }
    }
}

impl Graphics{
    pub fn keys_pressed(&self) -> KeysPressed{
        KeysPressed(())
    }
}
pub struct KeysPressed(());
impl Iterator for KeysPressed{
    type Item = Key;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe{
        match ffi::GetKeyPressed(){
            0 => None,
            x => Some(std::mem::transmute(x))
        }}
    }
}

impl Drop for Graphics {
    fn drop(&mut self) {
        unsafe{
            println!("DROP: Graphics");
            ffi::CloseWindow();
        }
    }
}

pub struct FileData{
    ptr: *mut u8,
    size: u32,
}
impl FileData {
    pub fn load(path: &str)->Option<FileData>{
        let mut size = 0;
        let ptr = unsafe {ffi::LoadFileData(ffi::rl_str!(path), &mut size)};
        if ptr.is_null(){
            None
        }else{
            Some(Self { ptr, size })
        }
    }
}
impl Drop for FileData {
    fn drop(&mut self) {
        println!("DROP: FileData");
        unsafe {ffi::UnloadFileData(self.ptr)}
    }
}

#[derive(Copy, Clone)]
pub struct SdfFont{
    font: ffi::Font,
}

#[derive(Copy, Clone)]
pub struct ImgFont{
    font: ffi::Font,
}

impl Graphics {
    pub fn font_from_file_sdf(&mut self, file: &FileData, glyph_count: i32) -> SdfFont{
        let base_size = 50;
        let padding = 0;
        let font = unsafe {
            let mut sdf_font = ffi::Font{
                baseSize: base_size,
                glyphCount: glyph_count,
                glyphPadding: 0,
                glyphs: ffi::LoadFontData(file.ptr, file.size as i32, base_size, 0 as *mut i32, glyph_count, FontType::Sdf as i32),
                texture: ffi::Texture { id: 0, width: 0, height: 0, mipmaps: 0, format: 0, },
                recs: 0 as *mut ffi::Rectangle,
            };
            let atlas = ffi::GenImageFontAtlas(sdf_font.glyphs, &mut sdf_font.recs, 95, base_size, padding, 1);
            sdf_font.texture = ffi::LoadTextureFromImage(atlas);
            ffi::SetTextureFilter(sdf_font.texture, TextureFilter::Bilinear as i32);    // Required for SDF font
            ffi::UnloadImage(atlas);
            sdf_font
        };
        SdfFont { font }
    }
    pub fn font_from_file_img(&mut self, file: &FileData, glyph_count: i32) -> ImgFont{
        let base_size = 15;
        let font = unsafe {
            let mut sdf_font = ffi::Font{
                baseSize: base_size,
                glyphCount: glyph_count,
                glyphPadding: 0,
                glyphs: ffi::LoadFontData(file.ptr, file.size as i32, base_size, 0 as *mut i32, glyph_count, FontType::Default as i32),
                texture: ffi::Texture { id: 0, width: 0, height: 0, mipmaps: 0, format: 0, },
                recs: 0 as *mut ffi::Rectangle,
            };
            let atlas = ffi::GenImageFontAtlas(sdf_font.glyphs, &mut sdf_font.recs, glyph_count, base_size, 0, 1);
            sdf_font.texture = ffi::LoadTextureFromImage(atlas);
            ffi::SetTextureFilter(sdf_font.texture, TextureFilter::Bilinear as i32);
            ffi::UnloadImage(atlas);
            sdf_font
        };
        ImgFont { font }
    }

    pub fn draw_text(&mut self, font: &SdfFont, text: &str, pos: Vec2, size: f32, color: ffi::Color){
        unsafe{ffi::DrawTextEx(font.font, 
                   ffi::rl_str!(text), 
                   pos.into(),
                   size,
                   0.0,
                   color)};
    }

    pub fn measure(&self, font: impl Borrow<SdfFont>, text: &str, size: f32) -> Vec2{
        unsafe{
            ffi::MeasureTextEx(font.borrow().font, ffi::rl_str!(text), size, 0.0)
        }.into()
    }
}

pub struct Shader(ffi::Shader);
impl Drop for Shader {
    fn drop(&mut self) {
        println!("DROP: Shader");
        unsafe{
            ffi::UnloadShader(self.0);
        }
    }
}

impl Graphics {
    pub fn load_shader(&mut self, vertex_shader_path: Option<&str>, frgment_shader_path: &str)-> Shader {
        unsafe{
            Shader(ffi::LoadShader(vertex_shader_path.map(|x|ffi::rl_str!(x)).unwrap_or(0 as *const i8), ffi::rl_str!(frgment_shader_path)))
        }
    }
    
}
