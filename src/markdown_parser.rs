use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use std::path::PathBuf;
use tokio::{fs::File, io::AsyncReadExt};

pub async fn parse_markdown(paths: &Vec<PathBuf>) {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    for path in paths {
        parse_file(&path, options).await;
    }
}

async fn parse_file(path: &PathBuf, parser_options: Options) {
    let mut f = match File::open(path).await {
        Ok(f) => f,
        Err(err) => panic!("parse_file markdown error: {:?}", err),
    };
    let mut markdown_content = String::new();
    match f.read_to_string(&mut markdown_content).await {
        Ok(_) => {}
        Err(err) => panic!("parse_file to string error: {:?}", err),
    };
    read_events(&path, &markdown_content, parser_options).await;
}

struct Task {
    text: String,
    file: PathBuf,
}

async fn read_events(path: &PathBuf,markdown_content: &str,parser_options: Options) {
    let parser = Parser::new_ext(markdown_content, parser_options);
    for event in parser {
        match event {
            Event::Start(tag) => {
                match tag {
                    Tag::Link {
                        link_type,
                        dest_url,
                        title,
                        id,
                    } => {
                    }
                    Tag::List(None) => {
                        println!("Unordered List Start");
                    }
                    // Let's only handle unordered lists like:
                    // - []
                    _ => {}
                }
            }
            Event::Text(text) => {
                // Process text events, you can implement your logic here
                println!("Event Text: {}", text);
            }
            Event::TaskListMarker(t) => {
                println!("Event tasklistmarker: {}", t);
            }
            Event::End(tag) => {
                match tag {
                    TagEnd::List(false) => {
                        println!("Unordered List End");
                    }
                    TagEnd::Item=>{}
                    TagEnd::Link=>{}
                    TagEnd::Paragraph=>{}
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
