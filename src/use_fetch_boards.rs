use crate::types::Board;
use gloo_console::log;
use gloo_net::http::Request;
use yew::prelude::*;

#[hook]
pub fn use_fetch_board() -> Vec<Board> {
    use crate::config::use_config;

    let config = use_config();
    let boards = use_state(|| vec![]);

    let url = format!("{}/boards", config.base_url);
    {
        let boards = boards.clone();
        use_effect_with((), move |_| {
            let boards = boards.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_boards: Vec<Board> = Request::get(&url)
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
