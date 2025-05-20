use crate::types::{Board, BoardInfo, Thread};
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

#[hook]
pub fn use_fetch_boards() -> Vec<BoardInfo> {
    use crate::config::use_config;

    let config = use_config();
    let boards = use_state(|| vec![]);

    let url = format!("{}/boards", config.base_url);
    {
        let boards = boards.clone();
        use_effect_with((), move |_| {
            let boards = boards.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_boards: Vec<BoardInfo> = Request::get(&url)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                boards.set(fetched_boards);
            });
            || ()
        });
    }
    boards.to_vec()
}

#[hook]
pub fn use_fetch_thread(slug: &String, id: &String) -> Thread {
    use crate::config::use_config;

    let config = use_config();
    let thread = use_state(|| Thread::default());

    let url = format!("{}/board?slug={}&id={}", config.base_url, slug, id);
    {
        let thread = thread.clone();
        use_effect_with((), move |_| {
            let thread = thread.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_thread: Thread = Request::get(&url)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                thread.set(fetched_thread);
            });
            || ()
        });
    }
    (*thread).clone()
}