use std::collections::HashSet;

enum MessagePart {
    Text(String),
    Image(String),
}

struct Message {
    moment: time::PrimitiveDateTime,
    parts: Vec<MessagePart>,
    author_name: String,
    id: Option<String>,
    reply_to_id: Option<String>,
}

struct Discussion {
    emoji: String,
    title: String,
    messages: Vec<Message>,
}

struct FullDiscussion {
    inner: Discussion,
    directory_name: String,
}

const MOMENT_FORMAT: &'static [time::format_description::FormatItem] =
    time::macros::format_description!("[day].[month].[year] [hour]:[minute]");

fn parse<'a>(mut lines: impl Iterator<Item = &'a str>) -> Discussion {
    let mut emoji = None;
    let mut title = None;
    let mut messages = Vec::new();
    'messages: loop {
        let mut lines_buf = Vec::new(); // technical
        let mut parts = Vec::new();
        let mut id = None;
        let mut reply_to_id = None;
        'parts: loop {
            struct EndData {
                author_name: String,
                moment: time::PrimitiveDateTime,
            }
            let mut end = None;
            let line = lines.next();
            if let Some(line) = line {
                if let Some(line) = line.strip_prefix('\\') {
                    if let Some(line) = line.strip_prefix('\\') {
                        lines_buf.push(line);
                    } else {
                        let mut arguments = line.split('\\');
                        let command_name = arguments.next().unwrap();
                        if command_name == "end" {
                            let author_name = arguments.next().unwrap().to_owned();
                            let moment = arguments.next().unwrap();
                            let moment =
                                time::PrimitiveDateTime::parse(&moment, &MOMENT_FORMAT).unwrap();
                            end = Some(EndData {
                                author_name,
                                moment,
                            });
                        } else if command_name == "title" {
                            title = Some(arguments.next().unwrap().to_owned());
                        } else if command_name == "emoji" {
                            emoji = Some(arguments.next().unwrap().to_owned());
                        } else if command_name == "image" {
                            parts.push(MessagePart::Image(arguments.next().unwrap().to_owned()));
                        } else if command_name == "id" {
                            id = Some(arguments.next().unwrap().to_owned());
                        } else if command_name == "reply to id" {
                            reply_to_id = Some(arguments.next().unwrap().to_owned());
                        }
                    }
                } else {
                    lines_buf.push(line);
                }
            }
            if (line.is_none() || end.is_some()) && !lines_buf.is_empty() {
                parts.push(MessagePart::Text(lines_buf.join("\n")));
                lines_buf.clear();
            }
            if let Some(end) = end {
                messages.push(Message {
                    parts,
                    moment: end.moment,
                    author_name: end.author_name,
                    id,
                    reply_to_id,
                });
                break 'parts;
            }
            if line.is_none() {
                break 'messages;
            }
        }
    }
    Discussion {
        emoji: emoji.unwrap(),
        title: title.unwrap(),
        messages,
    }
}

struct HTML {
    content: String,
}
impl HTML {
    fn new() -> Self {
        Self {
            content: String::from("<!DOCTYPE html>"),
        }
    }
    fn full_tag<'a>(
        mut self,
        name: &str,
        attrs: impl IntoIterator<Item = (&'a str, &'a str)>,
        contents: impl FnOnce(Self) -> Self,
    ) -> Self {
        self.content.push_str(&format!("<{}", name));
        for attr in attrs.into_iter() {
            self.content
                .push_str(&format!(" {}=\"{}\"", attr.0, attr.1));
        }
        self.content.push_str(&format!(">"));
        self = contents(self);
        self.content.push_str(&format!("</{}>", name));
        self
    }
    fn open_tag<'a>(
        mut self,
        name: &str,
        attrs: impl IntoIterator<Item = (&'a str, &'a str)>,
    ) -> Self {
        self.content.push_str(&format!("<{}", name));
        for attr in attrs.into_iter() {
            self.content
                .push_str(&format!(" {}=\"{}\"", attr.0, attr.1));
        }
        self.content.push_str(&format!(">"));
        self
    }
    fn text(mut self, text: &str) -> Self {
        self.content.push_str(&html_escape::encode_text(text));
        self
    }
    fn iter<I: IntoIterator>(mut self, it: I, mut f: impl FnMut(Self, I::Item) -> Self) -> Self {
        for i in it.into_iter() {
            self = f(self, i);
        }
        self
    }
    fn html(mut self, text: &str) -> Self {
        self.content.push_str(text);
        self
    }
    fn option<T>(mut self, o: &Option<T>, f: impl FnOnce(Self, &T) -> Self) -> Self {
        if let Some(t) = o {
            self = f(self, t);
        }
        self
    }
}

fn make_html(title: &str, styles: &str, body: impl FnOnce(HTML) -> HTML) -> String {
    HTML::new()
        .full_tag("html", [], |d| {
            d.full_tag("head", [], |d| {
                d.open_tag("meta", [("charset", "utf-8")])
                    .open_tag(
                        "meta",
                        [
                            ("name", "viewport"),
                            ("content", "width=device-width, initial-scale=1"),
                        ],
                    )
                    .full_tag("title", [], |d| d.text(title))
                    .open_tag("link", [("rel", "stylesheet"), ("href", styles)])
            })
            .full_tag("body", [], body)
        })
        .content
}

fn make_participant_html(participant_name: &str) -> String {
    let links_path = format!("participants/{}/links", participant_name);
    let links = std::fs::read_to_string(links_path).unwrap();
    make_html(participant_name, "../../participant.css", |d| {
        d.full_tag("h1", [("class", "title")], |d| d.text(participant_name))
            .open_tag(
                "img",
                [
                    ("class", "pfp"),
                    ("src", "pfp"),
                    (
                        "alt",
                        &format!("Profile picture of {}", participant_name)[..],
                    ),
                ],
            )
            .full_tag("ul", [], |d| {
                d.iter(links.lines(), |d, link| {
                    d.full_tag("li", [], |d| {
                        d.full_tag("a", [("href", link)], |d| d.text(link))
                    })
                })
            })
    })
}

fn make_discussion_html(discussion: &Discussion) -> String {
    make_html(&discussion.title, "../../discussion.css", |d| d.full_tag("h1", [("class", "title")], |d| d.text(&discussion.title)).iter(&discussion.messages, |d, message| message.))
}

struct DiscussionShort {
    directory_name: String,
    title: String,
    emoji: String,
    moment: time::PrimitiveDateTime,
    participants: HashSet<String>,
}

fn make_shorts(discussions: Vec<FullDiscussion>) -> Vec<DiscussionShort> {
    let mut shorts = Vec::new();
    for discussion in discussions {
        let max_moment = discussion
            .inner
            .messages
            .iter()
            .map(|message| message.moment)
            .max()
            .unwrap();
        let participants = discussion
            .inner
            .messages
            .into_iter()
            .map(|message| message.author_name)
            .collect();
        shorts.push(DiscussionShort {
            title: discussion.inner.title,
            emoji: discussion.inner.emoji,
            moment: max_moment,
            participants,
            directory_name: discussion.directory_name,
        });
    }
    shorts.sort_by_key(|short| short.moment);
    shorts
}

fn make_index_html(
    chunk: &[DiscussionShort],
    prev: &Option<String>,
    next: &Option<String>,
) -> String {
    make_html("Frozen Speech", "index.css", |d| {
        d.full_tag("h1", [("class", "title")], |d| d.text("Frozen Speech"))
            .full_tag("p", [("class", "description")], |d| {
                d.full_tag("a", [("href", "participants/megahomyak")], |d| {
                    d.text("megahomyak")
                })
                .text("'s valuable discussions archive")
            })
            .iter(chunk, |d, short| {
                d.full_tag("div", [("class", "discussion")], |d| {
                    d.full_tag("span", [("class", "emoji")], |d| d.text(&short.emoji))
                        .full_tag("div", [("class", "description")], |d| {
                            d.full_tag("p", [("class", "description")], |d| {
                                d.text(&short.moment.format(&MOMENT_FORMAT).unwrap())
                            })
                            .full_tag("h2", [("class", "title")], |d| {
                                d.full_tag(
                                    "a",
                                    [(
                                        "href",
                                        &format!("discussions/{}", short.directory_name)[..],
                                    )],
                                    |d| d.text(&short.title),
                                )
                            })
                            .full_tag(
                                "p",
                                [("class", "participants")],
                                |d| {
                                    d.text("Participants: ").html(
                                        &short
                                            .participants
                                            .iter()
                                            .map(|participant| {
                                                HTML::new()
                                                    .full_tag(
                                                        "a",
                                                        [(
                                                            "href",
                                                            &format!(
                                                                "participants/{}",
                                                                participant
                                                            )[..],
                                                        )],
                                                        |d| d.text(participant),
                                                    )
                                                    .content
                                            })
                                            .collect::<Vec<_>>()
                                            .join("\n"),
                                    )
                                },
                            )
                        })
                })
            })
            .full_tag("div", [("class", "navigation")], |d| {
                d.option(prev, |d, prev| {
                    d.full_tag("a", [("href", &prev[..])], |d| d.text("prev"))
                })
                .option(next, |d, next| {
                    d.full_tag("a", [("href", &next[..])], |d| d.text("next"))
                })
            })
    })
}

fn main() {
    for participant_dir in std::fs::read_dir("discussions").unwrap() {
        let participant_dir = participant_dir.unwrap();
        let participant_html = make_participant_html(participant_dir.file_name().to_str().unwrap());
        std::fs::write(participant_dir.path().join("index.html"), &participant_html).unwrap();
    }
    let mut discussions = Vec::new();
    for discussion_dir in std::fs::read_dir("discussions").unwrap() {
        let discussion_dir = discussion_dir.unwrap();
        let discussion_dir_path = discussion_dir.path();
        let contents_path = discussion_dir_path.join("contents");
        let contents = std::fs::read_to_string(contents_path).unwrap();
        let discussion = parse(contents.lines());
        let discussion_html = make_discussion_html(&discussion);
        discussions.push(FullDiscussion {
            inner: discussion,
            directory_name: discussion_dir.file_name().to_str().unwrap().to_owned(),
        });
        std::fs::write(discussion_dir_path.join("index.html"), &discussion_html).unwrap();
    }
    let shorts = make_shorts(discussions);
    let paginate = || shorts.chunks(10);
    let chunks_count = paginate().count();
    for (idx, chunk) in paginate().enumerate() {
        let count = idx + 1;
        let mut prev = None;
        if count != chunks_count {
            prev = Some(format!("page_{}", count + 1));
        }
        let mut next = None;
        if count != 1 {
            next = Some(format!("page_{}", count - 1));
        }
        let index_html = make_index_html(chunk, &prev, &next);
        if prev.is_none() {
            std::fs::write("index.html", &index_html).unwrap();
        }
        std::fs::write(format!("page_{}", count), &index_html).unwrap();
    }
}
