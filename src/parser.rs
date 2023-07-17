use crate::data_structures;
//(NOTE): limitations:
//-you can't have two words which has the same beginning (for example map and maple) because map will always register first
//-you can't even have two words with the same starting letter
//-you can't have mutliple digit numbers

pub struct ParserManager<T> {
    searched_words: Vec<WordProgress<T>>,
}

impl<T: Copy> ParserManager<T> {
    pub fn new(searched_words: Vec<WordProgress<T>>) -> Self {
        Self { searched_words }
    }

    pub fn add_word(&mut self, word: WordProgress<T>) {
        self.searched_words.push(word);
    }

    pub fn parse(&mut self) -> Option<data_structures::Queue<T>> {
        let mut line = String::new();
        let _byte_size = std::io::stdin().read_line(&mut line).unwrap();
        let command: String = line.chars().filter(|c| !c.is_whitespace()).collect();

        let mut queue: data_structures::Queue<T> = data_structures::Queue::new();

        for c in command.chars() {
            let mut msg: Option<WordProgressFeedback> = None;
            for searched_word in self.searched_words.iter_mut() {
                let return_val = searched_word.check_char(&c);

                match return_val {
                    Some(WordProgressFeedback::FinishedWord) => {
                        queue.push(searched_word.get_return_type());
                        msg = return_val;
                    }
                    Some(WordProgressFeedback::IncorrectCommand) => {
                        show_helpful_message(HelpfulMessage::IncorrectCommand);
                        self.reset();
                        return self.parse();
                    }
                    //(NOTE): None means that the wordProgress is freezed or the character is correct but it's not the start nor the end of the command
                    None => (),
                }
            }
            //(NOTE): reseting every wordProgress when one word is finished
            //we have to do this here cuz we're taking a mut reference to the searched_words
            if msg == Some(WordProgressFeedback::FinishedWord) {
                for searched_word in self.searched_words.iter_mut() {
                    searched_word.reset();
                }
            }
        }
        let mut freezed_count = 0;
        let mut count = 0;
        for searched_word in self.searched_words.iter() {
            count += 1;
            if searched_word.freeze {
                freezed_count += 1;
            }

            if searched_word.active {
                show_helpful_message(HelpfulMessage::CommandTypedIncorrectly);
                self.reset();
                return self.parse();
            }
        }
        if freezed_count == count {
            show_helpful_message(HelpfulMessage::IncorrectCommand);
            self.reset();
            return self.parse();
        }

        Some(queue)
    }

    pub fn reset(&mut self) {
        for searched_word in self.searched_words.iter_mut() {
            searched_word.reset();
        }
    }
}

#[derive(PartialEq)]
enum HelpfulMessage {
    IncorrectCommand,
    CommandTypedIncorrectly,
    GameTutorial,
}

#[derive(PartialEq)]
enum WordProgressFeedback {
    FinishedWord,
    IncorrectCommand,
}

pub struct WordProgress<T> {
    word: String,
    current_char: Option<char>,

    current_word_index: usize,
    word_size: usize,

    return_type: T,

    freeze: bool,
    active: bool,
}

enum CharacterValidation {
    CorrectCharacter,
    IncorrectCharacter,
    NoCharacter,
}

impl<T: Copy> WordProgress<T> {
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
            active: false,
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

    fn check_char(&mut self, c: &char) -> Option<WordProgressFeedback> {
        if self.freeze {
            return None;
        }

        match self
            .get_char_from_index(self.current_word_index)
            .equate_chars(c)
        {
            CharacterValidation::CorrectCharacter => {
                //println!("Correct on char {}", c);
                self.current_word_index += 1;

                //(NOTE): word_size check for single digit numbers
                if !self.active && self.word_size > 1 {
                    self.active = true;
                }

                if self.current_word_index == self.word_size {
                    self.active = false;
                    //println!("Finished the word {}", self.word);
                    return Some(WordProgressFeedback::FinishedWord);
                }
            }
            CharacterValidation::IncorrectCharacter => {
                if self.active {
                    return Some(WordProgressFeedback::IncorrectCommand);
                }
                //println!("Incorrect on char {}", c);
                self.freeze = true;
            }
            CharacterValidation::NoCharacter => {
                //(NOTE): we should never get to this state
                //println!("Finished a word ({})", self.word);
            }
        }
        None
    }

    fn reset(&mut self) {
        self.current_word_index = 0;
        self.freeze = false;
        self.active = false;
    }

    fn get_return_type(&self) -> T {
        self.return_type
    }
}

fn show_helpful_message(message: HelpfulMessage) {
    match message {
        HelpfulMessage::IncorrectCommand => {
            println!("---------------------------------");
            println!("CommandError: incorrect command");
            println!("\ncommands should look like this:\n2 right 5 up\nfor going 2 units right and 2 up\nfor more commands check \n'info' command or info in menu");
            println!("-------------------------------");
        }
        HelpfulMessage::CommandTypedIncorrectly => {
            println!("----------------------------------");
            println!("CommandError: command typed incorrectly");
            println!("\nupps, a misstroke \njust write the command again\nor check 'info' to see\nhow to write the commands");
            println!("-------------------------------");
        }
        HelpfulMessage::GameTutorial => {
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
        }
    }
}
