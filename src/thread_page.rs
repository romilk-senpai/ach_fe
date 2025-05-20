use crate::board_page::ThreadPost;
use crate::use_fetch_thread::use_fetch_thread;
use gloo_console::log;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ThreadPageProps {
    pub slug: String,
    pub id: String,
}

#[function_component(ThreadPage)]
pub fn thread_page(ThreadPageProps { slug, id }: &ThreadPageProps) -> Html {
    let thread = use_fetch_thread(&slug, &id);
    log!(&thread.op_post.created_at);
    html! {
        <div>
            <h1>{format!("Thread {} of {}", id, slug)}</h1>
            {if thread.op_post.id == 0 {
                html! { <p>{ "No thread found" }</p> }
            } else {
                html! { <ThreadPost thread={thread} slug={slug.clone()} /> }
            }}
        </div>
    }
}
