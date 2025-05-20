use crate::types::Thread;
use gloo_net::http::Request;
use yew::prelude::*;

#[hook]
pub fn use_fetch_thread(slug: &String, id: &String) -> Thread {
    use crate::config::use_config;

    let config = use_config();
    let thread = use_state(|| Thread::default());

    let url = format!("{}/board?slug={}&id={}", config.base_url, slug, id);
    {
        let thread = thread.clone();
        use_effect_with((), move |_| {
            let thread = thread.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_thread: Thread = Request::get(&url)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                thread.set(fetched_thread);
            });
            || ()
        });
    }
    (*thread).clone()
}
