mod app;
mod types;
mod use_fetch_boards;
use app::App;
use app::BoardsList;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/boards/:handle")]
    Boards { handle: String },
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
struct BoardProps {
    handle: String,
}

#[function_component(Board)]
fn board(BoardProps { handle }: &BoardProps) -> Html {
    let display_text = format!("You are looking at: /{}/", handle);
    html! {
        <>
            <BoardsList />
            <main>
                <h1>{display_text}</h1>
            </main>
        </>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <App /> },
        Route::Boards { handle } => html! { <Board handle={handle} /> },
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
