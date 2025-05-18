use crate::create_urbit_name::create_urbit_name;
use crate::types::{BoardExt, Thread};
use crate::use_fetch_board::use_fetch_board;
use crate::BoardsList;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct PostingFormProps {
    board: BoardExt,
}

#[function_component(PostingForm)]
fn posting_form(PostingFormProps { board }: &PostingFormProps) -> Html {
    let show_form = use_state(|| false);

    if *show_form {
        html! {
            <form class="posting-form">
                <div class="form-group">
                    <label for="name">{"Name"}</label>
                    <input type="text" id="name" name="name" value="Anonymous" />
                </div>
                <div class="form-group">
                    <label for="options">{"Options"}</label>
                    <input type="text" id="options" name="options" />
                </div>
                <div class="form-group">
                    <label for="subject">{"Subject"}</label>
                    <input type="text" id="subject" name="subject" />
                </div>
                <div class="form-group">
                    <label for="comment">{"Comment"}</label>
                    <textarea id="comment" name="comment" rows="5" />
                </div>
                <div class="form-group">
                    <label for="file">{"File"}</label>
                    <input type="file" id="file" name="file" />
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
pub struct BoardPageProps {
    pub slug: String,
}

#[function_component(BoardPage)]
pub fn board_page(BoardPageProps { slug }: &BoardPageProps) -> Html {
    let display_text = format!("You are looking at: /{}/", slug);
    let board = use_fetch_board(slug);
    html! {
        <>
            <BoardsList />
            <main>
                <h1>{display_text}</h1>
                <h2>{board.name.clone()}</h2>
                <p>{board.description.clone()}</p>
                <PostingForm board={board.clone()} />
                {if board.threads.len() > 0 {
                    board.threads.iter().map(|thread| {
                        html! { <ThreadPost thread={thread.clone()} /> }
                    }).collect::<Html>()
                } else {
                    html! { <p>{ "No threads found" }</p> }
                }}
            </main>
        </>
    }
}
