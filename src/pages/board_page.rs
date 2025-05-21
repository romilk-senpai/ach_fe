use crate::components::{BoardsList, PostingForm, ThreadPost};
use crate::hooks::use_fetch_board;
use crate::types::Board;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct StartNewThreadProps {
    board: Board,
}

#[function_component(StartNewThread)]
fn start_new_thread(StartNewThreadProps { board }: &StartNewThreadProps) -> Html {
    let show_form = use_state(|| false);

    if *show_form {
        html! {
            <PostingForm board={board.clone()} />
        }
    } else {
        html! {
            <h2 class="posting-form-show" onclick={move |_| show_form.set(true)}>{"[Start a New Thread]"}</h2>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct BoardPageProps {
    pub slug: String,
}

#[function_component(BoardPage)]
pub fn board_page(BoardPageProps { slug }: &BoardPageProps) -> Html {
    let board = use_fetch_board(slug);
    let display_text = format!("/{}/ - {}", slug, board.name.clone());
    html! {
        <>
            <BoardsList />
            <main>
                <h1>{display_text}</h1>
                <p>{board.description.clone()}</p>
                <StartNewThread board={board.clone()} />
                {if board.threads.len() > 0 {
                    board.threads.iter().map(|thread| {
                        html! { <ThreadPost thread={thread.clone()} slug={slug.clone()} /> }
                    }).collect::<Html>()
                } else {
                    html! { <p>{ "No threads found" }</p> }
                }}
            </main>
        </>
    }
}
