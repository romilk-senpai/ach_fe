use regex::Regex;

pub fn parse_text(text: &str) -> String {
    let mut html = String::new();
    let lines: Vec<&str> = text.lines().collect();
    let mut in_list = false;
    let mut in_ordered_list = false;
    let mut in_quote = false;
    let mut in_code = false;

    for line in lines {
        // Handle code blocks (4 spaces or tab)
        if line.starts_with("    ") || line.starts_with('\t') {
            if !in_code {
                html.push_str("<pre><code>");
                in_code = true;
            }
            html.push_str(&line.trim_start());
            html.push('\n');
            continue;
        } else if in_code {
            html.push_str("</code></pre>");
            in_code = false;
        }

        // Handle quotes
        if line.starts_with('>') {
            if !in_quote {
                html.push_str("<blockquote>");
                in_quote = true;
            }
            html.push_str(&line[1..].trim());
            html.push('\n');
            continue;
        } else if in_quote {
            html.push_str("</blockquote>");
            in_quote = false;
        } else {
            // Handle inline formatting for regular text
            html.push_str(&parse_inline(line));
            html.push('\n');
        }

        // Handle ordered lists
        if line.starts_with("1.") {
            if !in_ordered_list {
                html.push_str("<ol>");
                in_ordered_list = true;
            }
            html.push_str("<li>");
            html.push_str(&parse_inline(&line[2..].trim()));
            html.push_str("</li>");
            continue;
        } else if in_ordered_list {
            html.push_str("</ol>");
            in_ordered_list = false;
        }

        // Handle unordered lists
        if line.starts_with("* ") || line.starts_with("- ") {
            if !in_list {
                html.push_str("<ul>");
                in_list = true;
            }
            html.push_str("<li>");
            html.push_str(&parse_inline(&line[2..]));
            html.push_str("</li>");
            continue;
        } else if in_list {
            html.push_str("</ul>");
            in_list = false;
        }
    }

    // Close any open tags
    if in_code {
        html.push_str("</code></pre>");
    }
    if in_quote {
        html.push_str("</blockquote>");
    }
    if in_list {
        html.push_str("</ul>");
    }
    if in_ordered_list {
        html.push_str("</ol>");
    }

    html
}

pub fn parse_inline(text: &str) -> String {
    let mut result = text.to_string();

    // Handle escape sequences
    result = result
        .replace("\\n", "<br>")
        .replace("\\t", "&nbsp;&nbsp;&nbsp;&nbsp;")
        .replace("\\r", "")
        .replace("\\\\", "\\")
        .replace("\\*", "*")
        .replace("\\_", "_")
        .replace("\\`", "`")
        .replace("\\>", ">");

    // Handle spoilers (%%text%%)
    let spoiler_regex = Regex::new(r"%%(.*?)%%").unwrap();
    result = spoiler_regex.replace_all(&result, "<div class='spoiler'>$1</div>").to_string();

    // Handle bold (** or __)
    let bold_regex = Regex::new(r"\*\*(.*?)\*\*|__(.*?)__").unwrap();
    result = bold_regex.replace_all(&result, "<strong>$1$2</strong>").to_string();

    // Handle italic (* or _)
    let italic_regex = Regex::new(r"\*(.*?)\*|_(.*?)_").unwrap();
    result = italic_regex.replace_all(&result, "<em>$1$2</em>").to_string();

    // Handle code (`)
    let code_regex = Regex::new(r"`(.*?)`").unwrap();
    result = code_regex.replace_all(&result, "<code>$1</code>").to_string();

    // Handle post references (>>1)
    let post_regex = Regex::new(r">>(\d+)").unwrap();
    result = post_regex.replace_all(&result, "<a href='#post-$1'>>$1</a>").to_string();

    // Handle URLs
    let url_regex = Regex::new(r"https?://\S+").unwrap();
    result = url_regex.replace_all(&result, "<a href='$0'>$0</a>").to_string();

    result
}