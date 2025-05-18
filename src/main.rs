mod app;
mod types;
mod lib;
use crate::types::Thread;
use crate::lib::create_urbit_name;
mod use_fetch_boards;
mod use_fetch_board;
use use_fetch_board::use_fetch_board;
use app::App;
use app::BoardsList;
use yew::prelude::*;
use yew_router::prelude::*;
use chrono::{DateTime, TimeZone, Utc, NaiveDateTime};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/boards/:slug")]
    Boards { slug: String },
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Secure)]
fn secure() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ThreadPostProps {
    thread: Thread,
}

#[function_component(ThreadPost)]
fn thread_post(ThreadPostProps { thread }: &ThreadPostProps) -> Html {
    let op_image = "https://i.4cdn.org/k/1747432557629704s.jpg";
    let naive = NaiveDateTime::from_timestamp_opt(thread.timestamp, 0).unwrap();
    let datetime: DateTime<Utc> = TimeZone::from_utc_datetime(&Utc, &naive);
    let thread_date = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    let op_name = create_urbit_name();

    html! {
        <div class="thread-post">
            <div class="thread-post-op">
                <img alt="OP" src={op_image} loading="lazy" width="200" />
                <div class="thread-post-op-content">
                    <div class="thread-post-op-header">
                        <span class="thread-post-op-subject">{thread.subject.clone()}</span>
                        <span class="thread-post-op-name">{op_name}</span>
                        <span class="thread-post-op-timestamp">{thread_date}</span>
                        <span class="thread-post-op-num">{format!("№{}", thread.num)}</span>
                    </div>
                    <p>{thread.content.clone()}</p>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct BoardProps {
    slug: String,
}

#[function_component(Board)]
fn board(BoardProps { slug }: &BoardProps) -> Html {
    let display_text = format!("You are looking at: /{}/", slug);
    let board = use_fetch_board(slug);
    html! {
        <>
            <BoardsList />
            <main>
                <h1>{display_text}</h1>
                <h2>{board.name.clone()}</h2>
                <p>{board.description.clone()}</p>
                {if let Some(threads) = &board.threads {
                    threads.iter().map(|thread| {
                        html! { <ThreadPost thread={thread.clone()} /> }
                    }).collect::<Html>()
                } else {
                    html! { <p>{ "No threads found" }</p> }
                }}
            </main>
        </>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <App /> },
        Route::Boards { slug } => html! { <Board slug={slug} /> },
        Route::Secure => html! {
            <Secure />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}
fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<Main>::new().render();
}
