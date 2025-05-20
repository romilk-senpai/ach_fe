use crate::types::BoardInfo;
use crate::hooks::use_fetch_boards;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum LinkDisplay {
    Name,
    Slug,
}

#[derive(Properties, PartialEq)]
pub struct BoardsByCategoryProps {
    pub category: String,
    pub boards: Vec<BoardInfo>,
    pub link_display: LinkDisplay,
}

#[function_component(BoardsByCategory)]
pub fn boards_by_category(
    BoardsByCategoryProps {
        category,
        boards,
        link_display,
    }: &BoardsByCategoryProps,
) -> Html {
    fn get_link_display(board: &BoardInfo, link_display: &LinkDisplay) -> String {
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

    html! {
        <aside>
            <a href="/">{"Home"}</a>
            <BoardsByCategory category="Anime" boards={anime_boards} link_display={LinkDisplay::Slug} />
            <BoardsByCategory category="Misc." boards={misc_boards} link_display={LinkDisplay::Slug} />
        </aside>
    }
}

#[derive(Properties, PartialEq)]
pub struct BoardsNavigationProps {
    pub board_slugs: Vec<String>,
}

#[function_component(BoardsNavigation)]
pub fn boards_navigation(BoardsNavigationProps { board_slugs }: &BoardsNavigationProps) -> Html {
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