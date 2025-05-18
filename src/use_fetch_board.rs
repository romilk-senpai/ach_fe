use crate::types::Board;
use gloo_net::http::Request;
use yew::prelude::*;

#[hook]
pub fn use_fetch_board(slug: &String) -> Board {
    use crate::config::use_config;

    let config = use_config();
    let board = use_state(|| Board::default());

    let url = format!("{}/board?slug={}", config.base_url, slug);
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
