mod app;
mod components;
mod config;
mod helpers;
mod hooks;
mod pages;
mod quick_reply_component;
mod types;
use app::App;
use pages::*;
use quick_reply_component::*;
use types::{Board, Post};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/boards/:slug")]
    Boards { slug: String },
    #[at("/boards/:slug/thread/:id")]
    Thread { slug: String, id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <App /> },
        Route::Boards { slug } => html! { <BoardPage slug={slug} /> },
        Route::Thread { slug, id } => html! { <ThreadPage slug={slug} id={id} /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(Main)]
fn app() -> Html {
    let quick_reply_settings = use_state(|| QuickReplySettings {
        open: false,
        board: Board::default(),
        reply: None,
        thread_url: None,
    });
    let quick_reply_settings_clone = quick_reply_settings.clone();

    let quick_reply_context = QuickReplyContext {
        settings: (*quick_reply_settings).clone(),
        toggle: Callback::from(move |(reply, thread_url): (Post, String)| {
            quick_reply_settings.set(QuickReplySettings {
                open: !quick_reply_settings.open,
                board: Board::default(),
                reply: Some(reply),
                thread_url: Some(thread_url),
            });
        }),
        close: Callback::from(move |_: MouseEvent| {
            quick_reply_settings_clone.set(QuickReplySettings {
                open: false,
                board: Board::default(),
                reply: None,
                thread_url: None,
            });
        }),
    };

    html! {
        <ContextProvider<QuickReplyContext> context={quick_reply_context}>
            <BrowserRouter>
                <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
            </BrowserRouter>
            <QuickReply />
        </ContextProvider<QuickReplyContext>>
    }
}
fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<Main>::new().render();
}
