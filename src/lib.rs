use wasm_bindgen::prelude::*;
/*
use std::panic;
extern crate console_error_panic_hook;
panic::set_hook(Box::new(console_error_panic_hook::hook));
 */

extern crate nalgebra_glm as glm;

mod state;
mod util;
mod graphics;

use web_sys::{Document, HtmlImageElement};

use state::game_state::GameState;
use state::input_state::InputState;
use state::render_state::RenderState;

use util::logging::log;
use graphics::sprite::Sprite;

#[wasm_bindgen]
pub struct Game
{
    game_state: GameState,
    render_state: Option<RenderState>,
    input_state: InputState
}

#[wasm_bindgen]
impl Game
{
    pub fn new() -> Self
    {
        Self
        {
            game_state: GameState::new(),
            render_state: None::<RenderState>,
            input_state: InputState::new()
        }
    }

    pub fn init_renderer(&mut self, document: &Document)
    {
        self.render_state = RenderState::new(document);

    }
    
    pub fn load_shader(&mut self, vert_shader: &str, frag_shader: &str)
    {
        let render_state = match &mut self.render_state
        {
            Some(r) => {r}
            None => { return; }
        };

        render_state.set_shader(vert_shader, frag_shader);
    }

    pub fn load_texture(&mut self, img: HtmlImageElement)
    {
        let render_state = match &mut self.render_state
        {
            Some(r) => {r}
            None => { return; }
        };

        render_state.load_texture(img);
    }

    pub fn init_render_data(&mut self)
    {
        let render_state = match &mut self.render_state
        {
            Some(r) => {r}
            None => { return; }
        };

        render_state.submit_camera_uniforms(); //TODO: if we change perspective, do this more than once.

        //Possum!
        let sprite = Sprite::new([384,240],[0,0],[46,33],0,0,-1.0);

        //Background!
        let second_sprite = Sprite::new([1000,500],[0,0],[1000,500],1,1,-2.0); //TODO: z
        
        render_state.submit_data(&sprite);
        render_state.submit_data(&second_sprite);

        //TODO: later, move transform data somewhere else.
        let mut s_w: glm::Mat4 = glm::Mat4::identity().into();
        let mut s_2_w: glm::Mat4= glm::Mat4::identity().into();

        //NB: Z scale has to be 0 or we get clipped right now.
        let scale : glm::TVec3<f32> = glm::vec3(0.1,0.1,0.0);
        s_w = glm::scale(&s_w, &scale);

        let background_scale : glm::TVec3<f32> = glm::vec3(1.5,1.5,0.0);
        s_2_w = glm::scale(&s_2_w, &background_scale);

        let mut transform_uniform_data= Vec::<f32>::new();

        transform_uniform_data.extend_from_slice(s_w.as_slice());
        transform_uniform_data.extend_from_slice(s_2_w.as_slice());

        render_state.submit_transform_uniforms(&transform_uniform_data);

    }

    pub fn update(&mut self)
    {
        self.game_state.update(&self.input_state);
        //TODO: also update render state?
    }

    pub fn render(&self)
    {
        let render_state = match &self.render_state
        {
            Some(r) => {r}
            None => { return; }
        };

        render_state.clear_context();
        render_state.draw_buffer::<Sprite>();
    }

    pub fn process_event(&self, code : &str)
    {
        self.input_state.process_input(code);
    }
}