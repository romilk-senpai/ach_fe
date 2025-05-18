use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ThreadPageProps {
    pub slug: String,
    pub id: String,
}

#[function_component(ThreadPage)]
pub fn thread_page(ThreadPageProps { slug, id }: &ThreadPageProps) -> Html {
    html! {
        <div>
            <h1>{format!("Thread {} of {}", id, slug)}</h1>
        </div>
    }
}
