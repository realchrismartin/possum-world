import * as wasm from "possum_world"

export function init(textures)
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
        layout(location = 3) in float texture_index;

        uniform mat4 vp_matrix;
        uniform mat4 m_matrices[64];

        out vec2 vertex_texture_coordinates;
        out float vertex_texture_index;

        void main() 
        {
            gl_Position = m_matrices[int(model_matrix_index)] * vp_matrix * vec4(position,1.0);
            vertex_texture_coordinates = texture_coordinates;
            vertex_texture_index = texture_index;
        }
       `
    
    let frag_shader = `#version 300 es
    precision highp float;

    in vec2 vertex_texture_coordinates;
    in float vertex_texture_index;

    out vec4 outColor;
    uniform sampler2D u_texture_0;
    uniform sampler2D u_texture_1;

    void main() 
    {
        if( int(vertex_texture_index) == 0)
        {
            outColor = texture(u_texture_0, vertex_texture_coordinates);
        } else
        {
            outColor = texture(u_texture_1, vertex_texture_coordinates);
        }
    }
    `

   console.log(textures);
   render_state.set_shader(vert_shader,frag_shader);
   render_state.load_texture(textures[0]);
   render_state.load_texture(textures[1]);
   render_state.submit_sprite_data(); //For now, submit data once here.

    let eventArray = [];

    addEventListener("keydown",(event) => {
        eventArray.push(event.code);
    });

    let rotation = 0;

    const gameLoop = () =>
    {
        eventArray.forEach((event) =>{
           wasm.process_event(input_state,event);
        });
        eventArray = []

        rotation += .005;

        if(rotation > 360)
        {
            rotation = 0;
        }

        wasm.update(game_state,input_state);
        wasm.render(game_state,render_state,rotation);
        requestAnimationFrame(gameLoop);
    };

    requestAnimationFrame(gameLoop);
}