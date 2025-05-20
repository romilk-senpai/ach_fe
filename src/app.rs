use crate::types::BoardInfo;
use crate::hooks::use_fetch_boards;
use crate::components::{BoardsByCategory, BoardsNavigation, LinkDisplay};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let boards = use_fetch_boards();
    let get_boards_by_category = |category: &str| {
        boards
            .iter()
            .filter(|board| board.category_id == category)
            .cloned()
            .collect::<Vec<BoardInfo>>()
    };

    let anime_boards = get_boards_by_category("Japanese Culture");
    let misc_boards = get_boards_by_category("Interests");

    let get_board_slugs = || {
        boards
            .iter()
            .map(|board| board.slug.clone())
            .collect::<Vec<String>>()
    };

    let board_slugs = get_board_slugs();

    html! {
        <div class="container">
            <BoardsNavigation board_slugs={board_slugs.clone()} />
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
                <BoardsNavigation board_slugs={board_slugs.clone()} />
            </footer>
        </div>
    }
}
