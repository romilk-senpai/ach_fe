use crate::components::{BoardsList, ThreadPost};
use crate::hooks::{use_fetch_board, use_send_post_request};
use crate::types::{Board, FormInfo};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct PostingFormProps {
    board: Board,
}

#[function_component(PostingForm)]
fn posting_form(PostingFormProps { board }: &PostingFormProps) -> Html {
    let show_form = use_state(|| false);
    let form_info = use_state(|| FormInfo {
        // important: this is not working for an unknown reason
        slug: board.slug.clone(),
        name: "".to_string(),
        options: "".to_string(),
        subject: "".to_string(),
        comment: "".to_string(),
        file: "".to_string(),
    });
    let send_post = use_send_post_request((*form_info).clone());

    let onsubmit = {
        let send_post = send_post.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default(); /* Prevent event propagation */
            send_post.emit(());
        })
    };

    let oninput_field = {
        let form_info = form_info.clone();
        Callback::from(move |(e, field): (InputEvent, &str)| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*form_info).clone();
            match field {
                "name" => info.name = input.value(),
                "options" => info.options = input.value(),
                "subject" => info.subject = input.value(),
                "comment" => info.comment = input.value(),
                "file" => info.file = input.value(),
                _ => {}
            }
            form_info.set(info);
        })
    };

    let oninput_name = oninput_field.clone().reform(|e| (e, "name"));
    let oninput_options = oninput_field.clone().reform(|e| (e, "options"));
    let oninput_subject = oninput_field.clone().reform(|e| (e, "subject"));
    let oninput_comment = oninput_field.clone().reform(|e| (e, "comment"));
    let oninput_file = oninput_field.clone().reform(|e| (e, "file"));

    if *show_form {
        html! {
            <form class="posting-form" {onsubmit}>
                <div class="form-group">
                    <label for="name">{"Name"}</label>
                    <input type="text" id="name" name="name" value={form_info.name.clone()} oninput={oninput_name} />
                </div>
                <div class="form-group">
                    <label for="options">{"Options"}</label>
                    <input type="text" id="options" name="options" value={form_info.options.clone()} oninput={oninput_options} />
                </div>
                <div class="form-group">
                    <label for="subject">{"Subject"}</label>
                    <input type="text" id="subject" name="subject" value={form_info.subject.clone()} oninput={oninput_subject} />
                </div>
                <div class="form-group">
                    <label for="comment">{"Comment"}</label>
                    <textarea id="comment" name="comment" rows="5" value={form_info.comment.clone()} oninput={oninput_comment} />
                </div>
                <div class="form-group">
                    <label for="file">{"File"}</label>
                    <input type="file" id="file" name="file" value={form_info.file.clone()} oninput={oninput_file} />
                </div>
                <button type="submit">{"Post"}</button>
            </form>
        }
    } else {
        html! {
            <h2 class="posting-form-show" onclick={move |_| show_form.set(true)}>{"[Start a New Thread]"}</h2>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct BoardPageProps {
    pub slug: String,
}

#[function_component(BoardPage)]
pub fn board_page(BoardPageProps { slug }: &BoardPageProps) -> Html {
    let board = use_fetch_board(slug);
    let display_text = format!("/{}/ - {}", slug, board.name.clone());
    html! {
        <>
            <BoardsList />
            <main>
                <h1>{display_text}</h1>
                <p>{board.description.clone()}</p>
                <PostingForm board={board.clone()} />
                {if board.threads.len() > 0 {
                    board.threads.iter().map(|thread| {
                        html! { <ThreadPost thread={thread.clone()} slug={slug.clone()} /> }
                    }).collect::<Html>()
                } else {
                    html! { <p>{ "No threads found" }</p> }
                }}
            </main>
        </>
    }
}
