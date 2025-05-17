use gloo_net::http::Request;
use yew::prelude::*;
use crate::types::Board;

#[hook]
pub fn use_fetch_board(slug: &str) -> Board {
    let board = use_state(|| Board::default());

    let url = format!("http://localhost:3000/boards/{}", slug);

    {
        let board = board.clone();
        use_effect_with((), move |_| {
            let board = board.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_board: Board = Request::get(&url)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                board.set(fetched_board);
            });
            || ()
        });
    }
    (*board).clone()
}
