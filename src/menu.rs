use crate::{data_structures, parser};

pub struct MenuState {
    parser: parser::ParserManager<MenuCommand>,
}

#[derive(Clone, Copy)]
enum MenuCommand {
    Start,
    Quit,
}

impl MenuState {
    pub fn new() -> Self {
        let start = parser::WordProgress::new("play".to_string(), MenuCommand::Start);
        let quit = parser::WordProgress::new("quit".to_string(), MenuCommand::Quit);
        let mut searched_words = Vec::new();
        searched_words.push(start);
        searched_words.push(quit);
        let parser = parser::ParserManager::<MenuCommand>::new(searched_words);
        Self { parser }
    }

    pub fn update(&mut self) -> Option<String> {
        let queue: Option<data_structures::Queue<MenuCommand>> = self.parser.parse();
        match queue {
            Some(mut queue) => {
                for _ in 0..queue.size() {
                    let command = queue.pop();
                    match command {
                        MenuCommand::Start => {
                            println!("Type the filename of the level you want to play:");
                            let mut line = String::new();
                            let _ = std::io::stdin().read_line(&mut line).unwrap();
                            return Some(line);
                        }
                        MenuCommand::Quit => {
                            return Some("quit".to_string());
                        }
                    }
                }
            }
            None => (),
        }
        None
    }

    pub fn render(&self) {
        println!("-------------------");
        println!("        SAG");
        println!("simple ascii game");
        println!("-------------------");
        println!("        PLAY");
        println!("        QUIT");
    }
}
