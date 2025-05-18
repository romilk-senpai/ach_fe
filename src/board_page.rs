use crate::create_urbit_name::create_urbit_name;
use crate::types::Thread;
use crate::use_fetch_board::use_fetch_board;
use crate::BoardsList;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use yew::prelude::*;

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
