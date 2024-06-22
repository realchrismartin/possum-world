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
use graphics::renderable::RenderableConfig;
use graphics::sprite::Sprite;

#[wasm_bindgen]
pub struct Game
{
    game_state: GameState,
    render_state: Option<RenderState>,
    input_state: InputState,
    elapsed_ms: f32, //TODO
    sprites: Vec<Sprite> //TODO
}

#[wasm_bindgen]
impl Game
{
    pub fn new(document: &Document) -> Self
    {
        Self
        {
            game_state: GameState::new(),
            render_state: RenderState::new(document),
            input_state: InputState::new(),
            elapsed_ms: 0.0,
            sprites: Vec::new()
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

    pub fn load_texture(&mut self, index: u32, img: HtmlImageElement)
    {
        let render_state = match &mut self.render_state
        {
            Some(r) => {r}
            None => { return; }
        };

        render_state.load_texture(index,img);
    }

    pub fn init_render_data(&mut self)
    {
        let render_state = match &mut self.render_state
        {
            Some(r) => {r}
            None => { return; }
        };

        render_state.submit_camera_uniforms(); //TODO: if we change perspective, do this more than once.
    }

    pub fn init_game_data(&mut self)
    {
        let render_state = match &mut self.render_state
        {
            Some(r) => {r}
            None => { return; }
        };

        let possum_sprite_1 = match render_state.request_new_renderable::<Sprite>(&RenderableConfig::new([0,0],[376,192],0,-0.5))
        {
            Some(s) => s,
            None => { return; }
        };

        let bg_sprite = match render_state.request_new_renderable::<Sprite>(&RenderableConfig::new([0,0],[500,500],1,0.0))
        {
            Some(s) => s,
            None => { return; }
        };

        render_state.set_scale(&possum_sprite_1,glm::vec3(0.3,0.3,0.1));
        render_state.set_scale(&bg_sprite,glm::vec3(1.0,1.0,0.1));

        self.sprites.push(possum_sprite_1);
        self.sprites.push(bg_sprite);

    }

    pub fn update(&mut self, delta_time: f32)
    {
        let render_state = match &mut self.render_state
        {
            Some(r) => {r}
            None => { return; }
        };

        self.game_state.update(render_state, &self.input_state);

        self.elapsed_ms += delta_time;

        //TODO: not safe, temporary for testing
        render_state.set_rotation(&self.sprites[0], f32::sin(self.elapsed_ms / 1000.0));

        render_state.set_translation(&self.sprites[0], glm::vec3(0.0,f32::sin(self.elapsed_ms / 1000.0),-0.5));
    }

    pub fn render(&mut self)
    {
        let render_state = match &mut self.render_state
        {
            Some(r) => {r}
            None => { return; }
        };

        render_state.clear_context();
        render_state.submit_transform_buffer_uniforms();

        render_state.draw(&self.sprites);
    }

    pub fn process_event(&self, code : &str)
    {
        self.input_state.process_input(code);
    }
}