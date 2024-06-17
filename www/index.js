import * as wasm from "possum_world"

export function init(textureElement)
{
    let canvas = document.getElementById("canvas");

    canvas.height = document.documentElement.clientHeight; 
    canvas.width = document.documentElement.clientWidth;

    addEventListener("resize",(event) => {
        let canvas = document.getElementById("canvas");
        canvas.height = document.documentElement.clientHeight; 
        canvas.width = document.documentElement.clientWidth;
    });

    const input_state = wasm.new_input_state();
    const game_state = wasm.new_game_state();
    const render_state = wasm.new_render_state(document);

    if(render_state instanceof Error)
    {
        console.log(render_state);
        return;
    }
   
    let vert_shader = `#version 300 es
 
        layout(location = 0) in vec3 position;
        layout(location = 1) in float model_matrix_index;
        layout(location = 2) in vec2 texture_coordinates;

        uniform mat4 vp_matrix;
        uniform mat4 m_matrices[64];

        out vec2 vertex_texture_coordinates;

        void main() 
        {
            gl_Position = m_matrices[int(model_matrix_index)] * vp_matrix * vec4(position,1.0);
            vertex_texture_coordinates = texture_coordinates;
        }
       `
    
    let frag_shader = `#version 300 es
    precision highp float;
    in vec2 vertex_texture_coordinates;
    out vec4 outColor;
    uniform sampler2D u_texture;

    void main() 
    {
        outColor = texture(u_texture, vertex_texture_coordinates);
        //outColor = vec4(1,1,1,1);
    }
    `
   render_state.set_shader(vert_shader,frag_shader);
   render_state.set_texture(textureElement);

    let eventArray = [];

    addEventListener("keydown",(event) => {
        eventArray.push(event.code);
    });

    const gameLoop = () =>
    {
        eventArray.forEach((event) =>{
           wasm.process_event(input_state,event);
        });
        eventArray = []

        wasm.update(game_state,input_state);
        wasm.render(game_state,render_state);
        requestAnimationFrame(gameLoop);
    };

    requestAnimationFrame(gameLoop);
}