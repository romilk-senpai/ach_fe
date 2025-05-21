use crate::components::{PostingForm, PostingFormOptions};
use crate::types::{Board, Post};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct QuickReplySettings {
    pub open: bool,
    pub board: Board,
    pub reply: Option<Post>,
    pub thread_url: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct QuickReplyContext {
    pub settings: QuickReplySettings,
    pub toggle: Callback<(Post, String)>,
    pub close: Callback<MouseEvent>,
}

#[function_component(QuickReply)]
pub fn quick_reply_component() -> Html {
    let quick_reply_ctx = use_context::<QuickReplyContext>().expect("no ctx found");

    html! {
        if quick_reply_ctx.settings.open {
            <div id="quick-reply">
                {format!("Quick reply to {}", quick_reply_ctx.settings.thread_url.clone().unwrap_or_default())}
                <button onclick={quick_reply_ctx.close.clone()}>
                    {"Close"}
                </button>
                <PostingForm board={quick_reply_ctx.settings.board.clone()} options={PostingFormOptions { show_labels: false }} />
            </div>
        } else {
            <></>
        }
    }
}
