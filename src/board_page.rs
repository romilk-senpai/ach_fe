use crate::use_fetch_board::use_fetch_board;
use crate::BoardsList;
use crate::ThreadPost;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BoardPageProps {
    pub slug: String,
}

#[function_component(BoardPage)]
pub fn board_page(BoardPageProps { slug }: &BoardPageProps) -> Html {
    let display_text = format!("You are looking at: /{}/", slug);
    let board = use_fetch_board(slug);
    html! {
        <>
            <BoardsList />
            <main>
                <h1>{display_text}</h1>
                <h2>{board.name.clone()}</h2>
                <p>{board.description.clone()}</p>
                {if board.threads.len() > 0 {
                    board.threads.iter().map(|thread| {
                        html! { <ThreadPost thread={thread.clone()} /> }
                    }).collect::<Html>()
                } else {
                    html! { <p>{ "No threads found" }</p> }
                }}
            </main>
        </>
    }
}
