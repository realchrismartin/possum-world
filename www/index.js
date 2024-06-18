import {Game} from "possum_world"

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

    const game = Game.new();

    addEventListener("keydown",(event) => 
    {
        game.process_event(event.code);
    });

    //Set up the renderer with its shader and textures
    game.init_renderer(document);
    game.load_shader(vert_shader,frag_shader);

    for(const texture of textures)
    {
        game.load_texture(texture);
    }

    //Load initial data - has to be done after renderer is set up.
    game.init_render_data();

    //Run the game loop
    const gameLoop = () =>
    {
        game.update();
        game.render();

        requestAnimationFrame(gameLoop);
    };

    requestAnimationFrame(gameLoop);
}