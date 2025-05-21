use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct QuickReplySettings {
    pub open: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct QuickReplyContext {
    pub settings: QuickReplySettings,
    pub toggle: Callback<MouseEvent>,
}

#[function_component(QuickReply)]
pub fn quick_reply_component() -> Html {
    let quick_reply_ctx = use_context::<QuickReplyContext>().expect("no ctx found");

    html! {
        if quick_reply_ctx.settings.open {
            <div id="quick-reply">
                {"Quick reply"}
                <button onclick={quick_reply_ctx.toggle.clone()}>
                    {"Close"}
                </button>
            </div>
        } else {
            <></>
        }
    }
}
