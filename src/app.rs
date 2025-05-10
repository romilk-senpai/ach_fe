use gloo_console::log;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

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
fn boards_by_category(
    BoardsByCategoryProps { category, boards }: &BoardsByCategoryProps,
) -> Html {
    html! {
        <div class="card-body-section">
            <p>{category.clone()}</p>
            <ul>
                {for boards.iter().map(|board| {
                    html! { <li><a href={format!("https://boards.4chan.org/{}", board.handle)}>{board.name.clone()}</a></li> }
                })}
            </ul>
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let greet_input_ref = use_node_ref();

    let name = use_state(|| String::new());

    let greet_msg = use_state(|| String::new());
    {
        let greet_msg = greet_msg.clone();
        let name = name.clone();
        let name2 = name.clone();
        use_effect_with(name2, move |_| {
            spawn_local(async move {
                if name.is_empty() {
                    return;
                }

                let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &*name }).unwrap();
                // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
                let new_msg = invoke("greet", args).await.as_string().unwrap();
                greet_msg.set(new_msg);
            });

            || {}
        });
    }

    let greet = {
        let name = name.clone();
        let greet_input_ref = greet_input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            name.set(
                greet_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };

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

            <div class="row">
                <a href="https://tauri.app" target="_blank">
                    <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                </a>
                <a href="https://yew.rs" target="_blank">
                    <img src="public/yew.png" class="logo yew" alt="Yew logo"/>
                </a>
            </div>
            <p>{"Click on the Tauri and Yew logos to learn more."}</p>

            <form class="row" onsubmit={greet}>
                <input id="greet-input" ref={greet_input_ref} placeholder="Enter a name..." />
                <button type="submit">{"Greet"}</button>
            </form>
            <p>{ &*greet_msg }</p>
        </main>
    }
}
