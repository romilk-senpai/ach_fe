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

#[derive(Clone, PartialEq)]
enum LinkDisplay {
    Name,
    Handle,
}

#[derive(Properties, PartialEq)]
struct BoardsByCategoryProps {
    category: String,
    boards: Vec<Board>,
    link_display: LinkDisplay,
}

#[function_component(BoardsByCategory)]
fn boards_by_category(
    BoardsByCategoryProps {
        category,
        boards,
        link_display,
    }: &BoardsByCategoryProps,
) -> Html {
    fn get_link_display(board: &Board, link_display: &LinkDisplay) -> String {
        match link_display {
            LinkDisplay::Name => board.name.clone(),
            LinkDisplay::Handle => format!("/{}/", board.handle),
        }
    }

    html! {
        <div class="boards-card-body-section">
            <b>{category.clone()}</b>
            <ul>
                {for boards.iter().map(|board| {
                    html! { <li><a href={format!("/boards/{}", board.handle)}>{get_link_display(board, link_display)}</a></li> }
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
            <a href="/">{"Home"}</a>
            <BoardsByCategory category="Anime" boards={anime_boards} link_display={LinkDisplay::Handle} />
            <BoardsByCategory category="Misc." boards={misc_boards} link_display={LinkDisplay::Handle} />
        </aside>
    }
}

#[derive(Properties, PartialEq)]
struct BoardsNavigationProps {
    board_handles: Vec<String>,
}

#[function_component(BoardsNavigation)]
fn boards_navigation(BoardsNavigationProps { board_handles }: &BoardsNavigationProps) -> Html {
    html! {
        <nav class="boards-navigation">
            <span>{ "[" }</span>
            {for board_handles.iter().enumerate().map(|(index, board_handle)| {
                html! {
                    <>
                        <a href={format!("/boards/{}", board_handle)}>{board_handle}</a>
                        {if index < board_handles.len() - 1 {
                            html! { <span>{ " / " }</span> }
                        } else {
                            html! {}
                        }}
                    </>
                }
            })}
            <span>{ "]" }</span>
        </nav>
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

    let get_board_handles = || {
        boards
            .iter()
            .map(|board| board.handle.clone())
            .collect::<Vec<String>>()
    };

    let board_handles = get_board_handles();

    html! {
        <div class="container">
            <BoardsNavigation board_handles={board_handles.clone()} />
            <main>
                <div>
                    <p>{"What is Ach"}</p>
                    <p>{"Ach is a board built in Haskell and Rust (Tauri + Yew)"}</p>
                </div>
                <div class="boards-card">
                    <div class="boards-card-header"><h3>{"Boards:"}</h3></div>
                    <div class="boards-card-body">
                        <BoardsByCategory category="Anime" boards={anime_boards} link_display={LinkDisplay::Name} />
                        <BoardsByCategory category="Misc." boards={misc_boards} link_display={LinkDisplay::Name} />
                    </div>
                </div>
            </main>
            <footer>
                <BoardsNavigation board_handles={board_handles.clone()} />
            </footer>
        </div>
    }
}
