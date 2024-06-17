use crate::state::game_state::GameState;
use crate::state::input_state::InputState;
use crate::state::render_state::RenderState;

pub fn render_game(game_state : &GameState, render_state : &mut RenderState, angle: f32)
{
    let mut s_w: glm::Mat4 = glm::Mat4::identity().into();
    let s_2_w: glm::Mat4= glm::Mat4::identity().into();
    
    //let translation : glm::TVec3<f32> = glm::vec3(0.5,0.5,0.0);
    let axis : glm::TVec3<f32> = glm::vec3(0.0,0.0,1.0);
    s_w = glm::rotate(&s_w, angle,&axis);
    //s_w = glm::translate(&s_w,&translation);

    let mut transform_uniform_data= Vec::<f32>::new();

    transform_uniform_data.extend_from_slice(s_w.as_slice());
    transform_uniform_data.extend_from_slice(s_2_w.as_slice());

    render_state.submit_transform_uniforms(&transform_uniform_data);
    render_state.draw_sprites();
}

pub fn update_game(game_state : &mut GameState, input_state : &InputState)
{
    //log("Updated.");
    
}
