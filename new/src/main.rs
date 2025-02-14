enum MessagePart {
    Text(String),
    Image(String),
}

struct Message {
    date: String,
    parts: Vec<MessagePart>,
    author_name: String,
    id: Option<String>,
    reply_to_id: Option<String>,
}

struct Parsed {
    emoji: String,
    title: String,
    messages: Vec<Message>,
}

fn parse(inputfile: String) -> Parsed {
    let mut inputfile_lines = inputfile.lines();
    let mut messages = Vec::new();
    let mut title = None;
    let mut emoji = None;
    'message: loop {
        let mut message_parts = Vec::new();
        let mut id = None;
        let mut reply_to_id = None;
        let mut text_line = None;
        let mut end = false;
        let line = inputfile_lines.next();
        if let Some(line) = line {
            
                if let Some(line) = line.strip_prefix('\\') {
                    if let Some(line) = line.strip_prefix('\\') {
                        text_line = Some(line);
                    } else {
                        let mut arguments = line.split('\\');
                        let command_name = arguments.next().unwrap();
                        if command_name == "end" {
                            let author_name = inputfile_lines.next().unwrap();
                            let date = inputfile_lines.next().unwrap();
                            messages.push(Message {
                                parts: message_parts,
                                date: date.to_owned(),
                                author_name: author_name.to_owned(),
                                id,
                                reply_to_id,
                            });
                            end = true;
                        } else if command_name == "title" {
                            title = Some(arguments.next().unwrap().to_owned());
                        } else if command_name == "id" {
                            id = Some(arguments.next().unwrap().to_owned());
                        } else if command_name == "reply to id" {
                            reply_to_id = Some(arguments.next().unwrap().to_owned());
                        }
                    }
                } else {
                    text_line = Some(line);
                }
        }
        break;
    }
    Parsed {
        messages,
        title: title.unwrap(),
    }
}

fn main() {
    println!("Hello, world!");
}
