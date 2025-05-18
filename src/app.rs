use crate::types::Board;
use crate::use_fetch_boards::use_fetch_board;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
enum LinkDisplay {
    Name,
    Slug,
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
            LinkDisplay::Slug => format!("/{}/", board.slug),
        }
    }

    html! {
        <div class="boards-card-body-section">
            <b>{category.clone()}</b>
            <ul>
                {for boards.iter().map(|board| {
                    html! { <li><a href={format!("/boards/{}", board.slug)}>{get_link_display(board, link_display)}</a></li> }
                })}
            </ul>
        </div>
    }
}

#[function_component(BoardsList)]
pub fn boards_list() -> Html {
    let boards = use_fetch_board();

    let get_boards_by_category = |category: i32| {
        boards
            .iter()
            .filter(|board| board.category_id == category)
            .cloned()
            .collect::<Vec<Board>>()
    };

    let anime_boards = get_boards_by_category(1);
    let misc_boards = get_boards_by_category(2);

    html! {
        <aside>
            <a href="/">{"Home"}</a>
            <BoardsByCategory category="Anime" boards={anime_boards} link_display={LinkDisplay::Slug} />
            <BoardsByCategory category="Misc." boards={misc_boards} link_display={LinkDisplay::Slug} />
        </aside>
    }
}

#[derive(Properties, PartialEq)]
struct BoardsNavigationProps {
    board_slugs: Vec<String>,
}

#[function_component(BoardsNavigation)]
fn boards_navigation(BoardsNavigationProps { board_slugs }: &BoardsNavigationProps) -> Html {
    html! {
        <nav class="boards-navigation">
            <span>{ "[" }</span>
            {for board_slugs.iter().enumerate().map(|(index, board_slug)| {
                html! {
                    <>
                        <a href={format!("/boards/{}", board_slug)}>{board_slug}</a>
                        {if index < board_slugs.len() - 1 {
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
    let boards = use_fetch_board();
    let get_boards_by_category = |category: i32| {
        boards
            .iter()
            .filter(|board| board.category_id == category)
            .cloned()
            .collect::<Vec<Board>>()
    };

    let anime_boards = get_boards_by_category(1);
    let misc_boards = get_boards_by_category(2);

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
