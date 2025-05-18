use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;

#[derive(Clone, Deserialize, Default)]
pub struct Config {
    #[serde(rename = "baseUrl")]
    pub base_url: String,
}

#[hook]
pub fn use_config() -> Config {
    let config = use_state(|| Config::default());

    {
        let config = config.clone();
        use_effect_with((), move |_| {
            let config = config.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_config: Config = Request::get("./config/config.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                config.set(fetched_config);
            });
            || ()
        });
    }
    (*config).clone()
}
