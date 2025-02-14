struct Message {
    date: String,
    content: String,

}

fn parse(inputfile: String) -> Vec<Message> {
    let mut inputfile_lines = inputfile.lines();
    let mut messages = Vec::new();
    'message: loop {
        let mut message_lines = Vec::new();
        for line in &mut inputfile_lines {
            if let Some(line) = line.strip_prefix('\\') {
                if let Some(line) = line.strip_prefix('\\') {
                    message_lines.push(line);
                } else {
                    let mut arguments = line.split('\\');
                    let mut command_name = arguments.next();
                    command_name
                }
            }
        }
        break;
    }
    messages
}

fn main() {
    println!("Hello, world!");
}
