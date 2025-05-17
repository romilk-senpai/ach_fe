use gloo_net::http::Request;
use yew::prelude::*;
use crate::types::Board;
use gloo_console::log;

#[hook]
pub fn use_fetch_board() -> Vec<Board> {
    let boards = use_state(|| vec![]);
    {
        let boards = boards.clone();
        use_effect_with((), move |_| {
            let boards = boards.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_boards: Vec<Board> = Request::get("http://localhost:3000")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                log!("Hello", fetched_boards.as_ptr());
                boards.set(fetched_boards);
            });
            || ()
        });
    }
    boards.to_vec()
}
