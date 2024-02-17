use pulldown_cmark::{Event, Options, Parser, Tag};
use std::fs;
#[tokio::main]
async fn main() {
    let markdown_content = fs::read_to_string("../test.md").expect("Unable to read file");
    // Set up options and parser. Strikethroughs are not part of the CommonMark standard
    // and we therefore must enable it explicitly.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    let parser = Parser::new_ext(&markdown_content, options);
    // Iterate over the events produced by the parser
    for event in parser {
        match event {
            Event::Start(tag) => {
                
                println!("Event start obj tag: {:?}", tag);
                // Process start tag events, you can implement your logic here
                match tag {
                    Tag::Link { link_type, dest_url, title, id } =>{

                        println!("{:?}, {dest_url}, {title}, {id}",link_type);
                    }
                    Tag::List(None) => {
                        println!("Unordered List Start");
                    }
                    Tag::List(Some(1)) => {
                        println!("Ordered List Start");
                    }
                    // Add more cases based on your requirements
                    _ => {}
                }
            }
            Event::Text(text) => {
                // Process text events, you can implement your logic here
                println!("Event Text: {}", text);
            }
            Event::TaskListMarker(t)=>{

                println!("Event tasklistmarker: {}", t);
            }
            Event::End(tag) => {

                println!("Event end obj tag: {:?}", tag);
                // Process end tag events, you can implement your logic here
                //match tag {
                //    Tag::List(None) => {
                //        println!("Unordered List End");
                //    }
                //    Tag::List(Some(1)) => {
                //        println!("Ordered List End");
                //    }
                //    // Add more cases based on your requirements
                //    _ => {}
                //}
            }
            _ => {}
        }
    }
    //Cli::handle().await;
}
