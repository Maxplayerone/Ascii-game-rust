use crate::data_structures;

pub struct ParserManager<T>{
    searched_words: Vec<WordProgress<T>>,
    active_words: usize,
    freezed_words: usize,
}

impl<T:Copy> ParserManager<T>{
    pub fn new(searched_words: Vec<WordProgress<T>>) -> Self{
        Self{
            searched_words,
            active_words: 0,
            freezed_words: 0,
        }
    }
    
    pub fn add_word(&mut self, word: WordProgress<T>){
        self.searched_words.push(word);
    }
    
    pub fn parse(&mut self, command: String) -> Option<data_structures::Queue<T>>{
        let mut queue: data_structures::Queue<T> = data_structures::Queue::new();
        
        for c in command.chars() {
            let mut msg: Option<Message> = None;
            for searched_word in self.searched_words.iter_mut() {
                let return_val = searched_word.check_char(&c, &mut self.freezed_words);

                match return_val {
                    Some(Message::FinishedWord) => {
                        queue.push(searched_word.get_return_type());
                        msg = return_val;
                        if searched_word.size() > 1{
                            self.active_words -= 1;
                        }
                    }
                    Some(Message::StartedWord) => {
                        self.active_words += 1;
                    }
                    Some(Message::WordTypedIncorrectly) => {
                        show_helpful_message(Message::WordTypedIncorrectly);
                        return None;
                    },
                    _ => (),
                }
            }
            //println!("freezed words {} searched words {}", self.freezed_words, self.searched_words.len());
            if self.freezed_words == self.searched_words.len(){
                show_helpful_message(Message::IncorrectWord);
                return None;
            }

            //reseting every wordProgress when one word is finished
            if msg == Some(Message::FinishedWord) {
                for searched_word in self.searched_words.iter_mut() {
                    searched_word.reset();
                }
                self.freezed_words = 0;
            }
        }
        
        Some(queue)
    }
}

#[derive(PartialEq)]
enum Message {
    StartedWord,
    FinishedWord,
    IncorrectWord,
    WordTypedIncorrectly,
    GameTutorial,
    OnlyNumber,
}

fn show_helpful_message(message: Message) {
    match message {
        Message::OnlyNumber => {
            println!("-------------------------------");
            println!("CommandError: number only given");
            println!("\nAfter typing the number \nyou should type the command \nyou want to do \n(ex. 10 right)\n(moving right ten units)");
            println!("-------------------------------");
        }
        Message::IncorrectWord => {
            println!("-------------------------------");
            println!("CommandError: incorrect command");
            println!("\ncommands should look like this:\n2 right 5 up\nfor going 2 units right and 2 up\nfor more commands check \n'info' command or info in menu");
            println!("-------------------------------");
        }
        Message::WordTypedIncorrectly => {
            println!("-------------------------------");
            println!("CommandError: command typed incorrectly");
            println!("\nupps, a misstroke \njust write the command again\nor check 'info' to see\nhow to write the commands");
            println!("-------------------------------");
        }
        Message::GameTutorial => {
            println!("----------------------------------------------------");
            println!("                    Game tutorial");
            println!("\nIn this game you have a few commands incluing");
            println!("right - moving right");
            println!("left - moving left");
            println!("up - moving up");
            println!("down - moving down");
            println!("wait - waiting a game tick without doing anything");
            println!("info - showing this tutorial :>");
            println!("quit - quitting the game\n");
            println!("The game updates whenever you send a command\nwe call that game tick\n");
            println!("You can use a single command multiple times ex\n3 right 2 up\nmoves right 3 times and up 2 times");
            println!("----------------------------------------------------");
        },
        _ => ()
    }
}

//(NOTE): if we have two words with the same prefix (eg. map and maple) the shorter word will win
//for example with command maple the map will register and not the maple

pub struct WordProgress<T> {
    word: String,
    current_char: Option<char>,
    current_word_index: usize,
    word_size: usize,
    freeze: bool,
    started: bool,
    return_type: T,
}

enum CharacterValidation {
    CorrectCharacter,
    IncorrectCharacter,
    NoCharacter,
}

impl<T:Copy> WordProgress<T> {
    pub fn new(word: String, return_type: T) -> Self {
        let word = word;
        let word_size = word.chars().count();
        Self {
            word,
            return_type,
            current_char: None,
            current_word_index: 0,
            word_size,
            freeze: false,
            started: false,
        }
    }

    fn get_char_from_index(&mut self, index: usize) -> &mut Self {
        self.current_char = self.word.chars().nth(index);
        self
    }

    fn equate_chars(&self, c: &char) -> CharacterValidation {
        match self.current_char {
            Some(c2) => {
                if c2 == *c {
                    CharacterValidation::CorrectCharacter
                } else {
                    CharacterValidation::IncorrectCharacter
                }
            }
            None => CharacterValidation::NoCharacter,
        }
    }

    fn check_char(&mut self, c: &char, freezed_words: &mut usize) -> Option<Message> {
        if self.freeze {
            return None;
        }

        match self
            .get_char_from_index(self.current_word_index)
            .equate_chars(c)
        {
            CharacterValidation::CorrectCharacter => {
                self.current_word_index += 1;

                if !self.started && self.word_size > 1 {
                    self.started = true;
                    return Some(Message::StartedWord);
                }

                if self.current_word_index == self.word_size {
                    //println!("Finished the word {}", self.word);
                    return Some(Message::FinishedWord);
                }
            }
            CharacterValidation::IncorrectCharacter => {
                self.freeze = true;
                *freezed_words += 1;
            }
            CharacterValidation::NoCharacter => {
                //println!("Finished a word ({})", self.word);
                return Some(Message::FinishedWord);
            }
        }
        None
    }

    fn reset(&mut self) {
        self.current_word_index = 0;
        self.freeze = false;
        self.started = false;
    }

    fn get_return_type(&self) -> T {
        self.return_type
    }

    fn size(&self) -> usize{
        self.word_size
    }
}

