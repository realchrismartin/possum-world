import * as wasm from "possum_world"

export function init()
{
    const input_state = wasm.new_input_state();
    const game_state = wasm.new_game_state();
    const render_state = wasm.new_render_state();

    render_state.set_context(document);

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