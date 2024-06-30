import {Game} from "possum_world"

export function init(textures,shader_sources)
{
    let canvas = document.getElementById("canvas");

    //TODO
    canvas.height = "1000"; // document.documentElement.clientHeight; 
    canvas.width = "1000"; //document.documentElement.clientWidth;

    addEventListener("resize",(event) => {
        let canvas = document.getElementById("canvas");
        //canvas.height = document.documentElement.clientHeight; 
        //canvas.width = document.documentElement.clientWidth;
    });

    const game = Game.new(document);

    //Attach event listeners for keypresses
    addEventListener("keydown",(event) => 
    {
        game.process_keypress_event(true,event.code);
    });

    addEventListener("keyup",(event) => 
    {
        game.process_keypress_event(false,event.code);
    });

    canvas.addEventListener("mousedown",(event) =>
    {
        game.process_click_event(true,event.offsetX,event.offsetY);
    });

    canvas.addEventListener("mousemove",(event) =>
    {
        game.process_mouse_move_event(event.offsetX,event.offsetY);
    });

    addEventListener("mouseup", (event) => 
    {
        game.process_click_event(false,event.offsetX,event.offsetY);
    });
    
    //Load shader
    //TODO: allow loading more than one shader
    game.load_shader(shader_sources[0],shader_sources[1]);

    //Load textures
    let index = 0;
    for(const texture of textures)
    {
        game.load_texture(index,texture);
        index = index + 1;
    }

    //Load initial data - has to be done after renderer is set up.
    game.init_game_data();

    //TODO: make the clock less rudimentary
    let now = new Date();

    //Run the game loop
    const gameLoop = () =>
    {
        let previous = now;
        now = new Date();

        let delta_time = (now - previous); //in MS

        game.update(delta_time);
        game.render();

        requestAnimationFrame(gameLoop);
    };

    requestAnimationFrame(gameLoop);
}