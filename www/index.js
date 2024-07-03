import {Game} from "possum_world"

export function init(textures,shader_sources)
{
    const game = Game.new(document);

    let setCanvasSizeFn = (game_object) => 
    {
        let container = document.getElementById("canvas-container");
        let canvas = document.getElementById("canvas");

         //Ensure the canvas is sized according to powers of 2 to prevent visual artifacts
         let x = Math.pow(2, parseInt(Math.log(container.clientWidth) / Math.log(2), 10));
         let y = Math.pow(2, parseInt(Math.log(container.clientHeight) / Math.log(2), 10));

        canvas.width = x; 
        canvas.height = y; 

        game_object.set_canvas_dimensions(x,y);
    };

    addEventListener("resize",(event) =>
    {
        setCanvasSizeFn(game);
    });

    setCanvasSizeFn(game);

    //Attach event listeners for keypresses
    addEventListener("keydown",(event) => 
    {
        game.process_keypress_event(true,event.code);
    });

    addEventListener("keyup",(event) => 
    {
        game.process_keypress_event(false,event.code);
    });

    canvas.addEventListener("pointerdown",(event) =>
    {
        game.process_click_event(true,event.offsetX,event.offsetY);
    });

    canvas.addEventListener("pointermove",(event) =>
    {
        game.process_mouse_move_event(event.offsetX,event.offsetY);
    });

    addEventListener("pointerup", (event) => 
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