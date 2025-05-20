use crate::board_page::ThreadPost;
use crate::components::BoardsList;
use crate::hooks::use_fetch_thread;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ThreadPageProps {
    pub slug: String,
    pub id: String,
}

#[function_component(ThreadPage)]
pub fn thread_page(ThreadPageProps { slug, id }: &ThreadPageProps) -> Html {
    let thread = use_fetch_thread(&slug, &id);
    let display_text = format!("/{}/ - {}", slug, thread.op_post.id.clone());
    html! {
        <>
            <BoardsList />
            <main>
                <h1>{display_text}</h1>
                {if thread.op_post.id == 0 {
                    html! { <p>{ "No thread found" }</p> }
                } else {
                    html! { <ThreadPost thread={thread} slug={slug.clone()} /> }
                }}
            </main>
        </>
    }
}
