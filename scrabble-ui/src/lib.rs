// #![cfg(target_arch = "wasm32")]

mod board;
mod js;
mod tile;
mod webgl_test;

use board::Board;
use wasm_bindgen::prelude::*;
use webgl_test::WebGlTest;
use yew::{html, Html};

#[wasm_bindgen(start)]
pub fn run() {
    yew::Renderer::<App>::new().render();
}

#[yew::function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="application">
            <h1 class="title">{ "scrabble" }</h1>
            <WebGlTest />
            <Board />
        </div>
    }
}
