use gloo_console::log;
use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;

#[derive(Clone, PartialEq, Deserialize)]
struct Board {
    name: String,
    category: String,
    handle: String,
}

#[derive(Properties, PartialEq)]
struct BoardsByCategoryProps {
    category: String,
    boards: Vec<Board>,
}

#[function_component(BoardsByCategory)]
fn boards_by_category(BoardsByCategoryProps { category, boards }: &BoardsByCategoryProps) -> Html {
    html! {
        <div class="card-body-section">
            <p>{category.clone()}</p>
            <ul>
                {for boards.iter().map(|board| {
                    html! { <li><a href={format!("/boards/{}", board.handle)}>{board.name.clone()}</a></li> }
                })}
            </ul>
        </div>
    }
}

#[function_component(BoardsList)]
pub fn boards_list() -> Html {
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
                boards.set(fetched_boards);
            });
            || ()
        });
    }

    let get_boards_by_category = |category: &str| {
        boards
            .iter()
            .filter(|board| board.category == category)
            .cloned()
            .collect::<Vec<Board>>()
    };

    let anime_boards = get_boards_by_category("anime");
    let misc_boards = get_boards_by_category("Misc.");

    html! {
        <aside>
            <BoardsByCategory category="Anime" boards={anime_boards} />
            <BoardsByCategory category="Misc." boards={misc_boards} />
        </aside>
    }
}

#[function_component(App)]
pub fn app() -> Html {
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

    let get_boards_by_category = |category: &str| {
        boards
            .iter()
            .filter(|board| board.category == category)
            .cloned()
            .collect::<Vec<Board>>()
    };

    let anime_boards = get_boards_by_category("anime");
    let misc_boards = get_boards_by_category("Misc.");

    html! {
        <main class="container">
            <div>
                <p>{"What is Ach"}</p>
                <p>{"Ach is a board built in Haskell and Rust (Tauri + Yew)"}</p>
            </div>

            <div class="card">
                <p class="card-header">{"Boards:"}</p>
                <div class="card-body">
                    <BoardsByCategory category="Anime" boards={anime_boards} />
                    <BoardsByCategory category="Misc." boards={misc_boards} />
                </div>
            </div>
        </main>
    }
}
