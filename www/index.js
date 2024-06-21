import {Game} from "possum_world"

export function init(textures,shader_sources)
{
    let canvas = document.getElementById("canvas");

    canvas.height = document.documentElement.clientHeight; 
    canvas.width = document.documentElement.clientWidth;

    addEventListener("resize",(event) => {
        let canvas = document.getElementById("canvas");
        canvas.height = document.documentElement.clientHeight; 
        canvas.width = document.documentElement.clientWidth;
    });

    const game = Game.new(document);

    addEventListener("keydown",(event) => 
    {
        game.process_event(event.code);
    });
    
    //TODO: allow loading more than one shader
    game.load_shader(shader_sources[0],shader_sources[1]);

    let index = 0;
    for(const texture of textures)
    {
        game.load_texture(index,texture);
        index++;
    }

    //Load initial data - has to be done after renderer is set up.
    game.init_render_data();
    game.init_game_data();

    //Run the game loop
    const gameLoop = () =>
    {
        game.update();
        game.render();

        requestAnimationFrame(gameLoop);
    };

    requestAnimationFrame(gameLoop);
}