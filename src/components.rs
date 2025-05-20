use crate::helpers::{create_urbit_name, parse_text, transform_date, HtmlToYew};
use crate::hooks::use_fetch_boards;
use crate::types::{BoardInfo, Post, Thread};
use yew::prelude::*;
use gloo_console::log;

#[derive(Clone, PartialEq)]
pub enum LinkDisplay {
    Name,
    Slug,
}

#[derive(Properties, PartialEq)]
pub struct BoardsByCategoryProps {
    pub category: String,
    pub boards: Vec<BoardInfo>,
    pub link_display: LinkDisplay,
}

#[function_component(BoardsByCategory)]
pub fn boards_by_category(
    BoardsByCategoryProps {
        category,
        boards,
        link_display,
    }: &BoardsByCategoryProps,
) -> Html {
    fn get_link_display(board: &BoardInfo, link_display: &LinkDisplay) -> String {
        match link_display {
            LinkDisplay::Name => board.name.clone(),
            LinkDisplay::Slug => format!("/{}/", board.slug),
        }
    }

    html! {
        <div class="boards-card-body-section">
            <b>{category.clone()}</b>
            <ul>
                {for boards.iter().map(|board| {
                    html! { <li><a href={format!("/boards/{}", board.slug)}>{get_link_display(board, link_display)}</a></li> }
                })}
            </ul>
        </div>
    }
}

#[function_component(BoardsList)]
pub fn boards_list() -> Html {
    let boards = use_fetch_boards();

    let get_boards_by_category = |category: &str| {
        boards
            .iter()
            .filter(|board| board.category_id == category)
            .cloned()
            .collect::<Vec<BoardInfo>>()
    };

    let anime_boards = get_boards_by_category("Japanese Culture");
    let misc_boards = get_boards_by_category("Interests");

    html! {
        <aside>
            <a href="/">{"Home"}</a>
            <BoardsByCategory category="Anime" boards={anime_boards} link_display={LinkDisplay::Slug} />
            <BoardsByCategory category="Misc." boards={misc_boards} link_display={LinkDisplay::Slug} />
        </aside>
    }
}

#[derive(Properties, PartialEq)]
pub struct BoardsNavigationProps {
    pub board_slugs: Vec<String>,
}

#[function_component(BoardsNavigation)]
pub fn boards_navigation(BoardsNavigationProps { board_slugs }: &BoardsNavigationProps) -> Html {
    html! {
        <nav class="boards-navigation">
            <span>{ "[" }</span>
            {for board_slugs.iter().enumerate().map(|(index, board_slug)| {
                html! {
                    <>
                        <a href={format!("/boards/{}", board_slug)}>{board_slug}</a>
                        {if index < board_slugs.len() - 1 {
                            html! { <span>{ " / " }</span> }
                        } else {
                            html! {}
                        }}
                    </>
                }
            })}
            <span>{ "]" }</span>
        </nav>
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

    let content = parse_text(&op_post.content);
    log!(&content);

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
                    <HtmlToYew html={content.clone()} />
                </div>
            </div>
            <LastReplies last_replies={thread.last_replies.clone()} thread_url={thread_url.clone()} />
        </div>
    }
}
