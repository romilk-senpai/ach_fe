mod app;
mod board_page;
mod config;
mod create_urbit_name;
mod thread_page;
mod transform_date;
mod types;
mod use_fetch_board;
mod use_fetch_boards;
mod use_fetch_thread;
use app::App;
use app::BoardsList;
use board_page::BoardPage;
use thread_page::ThreadPage;
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
