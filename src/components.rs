use crate::helpers::{create_urbit_name, parse_text, transform_date, HtmlToYew};
use crate::hooks::{use_fetch_boards, use_send_post_request};
use crate::quick_reply_component::*;
use crate::types::{Board, BoardInfo, FormInfo, Post, Thread};
use gloo_console::log;
use web_sys::HtmlInputElement;
use yew::prelude::*;

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

    let quick_reply_ctx = use_context::<QuickReplyContext>().expect("no ctx found");

    let reply_url = format!("{}#{}", thread_url, reply.id);

    let open_quick_reply = {
        let reply = reply.clone();
        let thread_url = thread_url.clone();
        quick_reply_ctx.toggle.reform(move |_| (reply.clone(), thread_url.clone()))
    };

    html! {
        <div class="thread-post-reply">
            <img alt="reply" src={"mock"} loading="lazy" width="200" />
            <div class="thread-post-op-content">
                <div class="thread-post-op-header">
                    <span class="thread-post-op-subject">{reply.subject.clone()}</span>
                    <span class="thread-post-op-name">{reply_name}</span>
                    <span class="thread-post-op-timestamp">{reply_date}</span>
                    <span
                       class="thread-post-op-num"
                       onclick={open_quick_reply}>
                        {format!("№{}", reply.id)}
                    </span>
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
                    <div class="thread-post-op-content-inner">
                        <HtmlToYew html={content.clone()} />
                    </div>
                </div>
            </div>
            <LastReplies last_replies={thread.last_replies.clone()} thread_url={thread_url.clone()} />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct PostingFormOptions {
    pub show_labels: bool,
}

#[derive(Properties, PartialEq)]
pub struct PostingFormProps {
    pub board: Board,
    #[prop_or_default]
    pub options: Option<PostingFormOptions>,
}

#[function_component(PostingForm)]
pub fn posting_form(PostingFormProps { board, options }: &PostingFormProps) -> Html {
    let show_labels = options.as_ref().map_or(true, |o| o.show_labels);
    let form_class = if show_labels {
        "posting-form"
    } else {
        "posting-form no-labels"
    };

    let form_info = use_state(|| FormInfo {
        // important: this is not working for an unknown reason
        slug: board.slug.clone(),
        name: "".to_string(),
        options: "".to_string(),
        subject: "".to_string(),
        content: "".to_string(),
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
                "content" => info.content = input.value(),
                "file" => info.file = input.value(),
                _ => {}
            }
            form_info.set(info);
        })
    };

    let oninput_name = oninput_field.clone().reform(|e| (e, "name"));
    let oninput_options = oninput_field.clone().reform(|e| (e, "options"));
    let oninput_subject = oninput_field.clone().reform(|e| (e, "subject"));
    let oninput_content = oninput_field.clone().reform(|e| (e, "content"));
    let oninput_file = oninput_field.clone().reform(|e| (e, "file"));

    html! {
        <form class={form_class} {onsubmit}>
            <div class="form-group">
                <label for="name">{"Name"}</label>
                <input type="text" id="name" name="name" placeholder="Name" value={form_info.name.clone()} oninput={oninput_name} />
            </div>
            <div class="form-group">
                <label for="options">{"Options"}</label>
                <input type="text" id="options" name="options" placeholder="Options" value={form_info.options.clone()} oninput={oninput_options} />
            </div>
            <div class="form-group">
                <label for="subject">{"Subject"}</label>
                <input type="text" id="subject" name="subject" placeholder="Subject" value={form_info.subject.clone()} oninput={oninput_subject} />
            </div>
            <div class="form-group">
                <label for="content">{"content"}</label>
                <textarea id="content" name="content" rows="5" placeholder="content" value={form_info.content.clone()} oninput={oninput_content} />
            </div>
            <div class="form-group">
                <label for="file">{"File"}</label>
                <input type="file" id="file" name="file" placeholder="File" value={form_info.file.clone()} oninput={oninput_file} />
            </div>
            <button type="submit">{"Post"}</button>
        </form>
    }
}