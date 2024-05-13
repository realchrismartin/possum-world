import * as wasm from "possum_world"

export function init()
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
 
        layout(location = 0) in vec4 position;
        layout(location = 1) in float model_matrix_index;

        uniform mat4 vp_matrix;
        uniform mat4 m_matrices[64];

        void main() 
        {
            gl_Position = m_matrices[int(model_matrix_index)] * vp_matrix * position;
        }
       `
    
    let frag_shader = `#version 300 es
    precision highp float;
    out vec4 outColor;
    
    void main() {
        outColor = vec4(1, 1, 1, 1);
    }
    `
   render_state.set_shader(vert_shader,frag_shader);

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