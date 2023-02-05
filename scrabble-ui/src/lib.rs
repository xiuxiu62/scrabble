// #![cfg(target_arch = "wasm32")]

use std::path::PathBuf;

use scrabble_core::board::Board;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
pub fn app() -> Html {
    // let board: Option<Board<15>> = match Board::try_from(PathBuf::from("boards/default-15x15.txt"))
    // {
    //     Ok(board) => Some(board),
    //     Err(err) => {
    //         log(err.to_string().as_str());

    //         None
    //     }
    // };

    let board: Board<15> = Board::default();
    log(format!("{board:?}").as_str());

    html! {
        <div>
            <h1>{ "Hello world" }</h1>
            <p>{ "look at this graph!" }</p>
            <p>{{ format!("{board:?}") }}</p>
        </div>
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
