use crate::components::BoardsList;
use crate::helpers::{create_urbit_name, transform_date};
use crate::hooks::{use_fetch_board, use_send_post_request};
use crate::types::{Board, FormInfo, Post, Thread};
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
struct ReplyProps {
    reply: Post,
    thread_url: String,
}

#[function_component(Reply)]
fn reply(ReplyProps { reply, thread_url }: &ReplyProps) -> Html {
    let reply_date = transform_date(&reply.created_at);
    let reply_name = match &reply.author {
        author if !author.is_empty() => author.clone(),
        _ => create_urbit_name(),
    };

    let reply_url = format!("{}#{}", thread_url, reply.id);

    html! {
        <div class="thread-post-reply">
            <img alt="reply" src={"mock"} loading="lazy" width="200" />
            <div class="thread-post-op-content">
            <div class="thread-post-op-header">
                <span class="thread-post-op-subject">{reply.subject.clone()}</span>
                <span class="thread-post-op-name">{reply_name}</span>
                <span class="thread-post-op-timestamp">{reply_date}</span>
                <a href={reply_url.clone()} class="thread-post-op-num">{format!("№{}", reply.id)}</a>
                </div>
                <p>{reply.content.clone()}</p>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct LastRepliesProps {
    last_replies: Vec<Post>,
    thread_url: String,
}

#[function_component(LastReplies)]
fn last_replies(
    LastRepliesProps {
        last_replies,
        thread_url,
    }: &LastRepliesProps,
) -> Html {
    html! {
        <div class="thread-post-replies">
            {if last_replies.len() > 0 {
                last_replies.iter().map(|reply| {
                    html! { <Reply reply={reply.clone()} thread_url={thread_url.clone()} /> }
                }).collect::<Html>()
            } else {
                html! { <></> }
            }}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ThreadPostProps {
    pub thread: Thread,
    pub slug: String,
}

#[function_component(ThreadPost)]
pub fn thread_post(ThreadPostProps { thread, slug }: &ThreadPostProps) -> Html {
    let op_post = thread.op_post.clone();
    let op_image = "https://i.4cdn.org/k/1747432557629704s.jpg";
    let thread_date = transform_date(&op_post.created_at);
    let op_name = match &op_post.author {
        author if !author.is_empty() => author.clone(),
        _ => create_urbit_name(),
    };

    let thread_url = format!("/boards/{}/thread/{}", slug, op_post.id);

    html! {
        <div class="thread-post">
            <div class="thread-post-op">
                <img alt="OP" src={op_image} loading="lazy" width="200" />
                <div class="thread-post-op-content">
                    <div class="thread-post-op-header">
                        <span class="thread-post-op-subject">{op_post.subject}</span>
                        <span class="thread-post-op-name">{op_name}</span>
                        <span class="thread-post-op-timestamp">{thread_date}</span>
                        <a href={thread_url.clone()} class="thread-post-op-num">{format!("№{}", op_post.id)}</a>
                    </div>
                    <p>{op_post.content}</p>
                </div>
            </div>
            <LastReplies last_replies={thread.last_replies.clone()} thread_url={thread_url.clone()} />
        </div>
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
