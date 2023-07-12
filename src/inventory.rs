use crate::data_structures;
use crate::{ItemType, LocationType};

struct ItemDescriptor {
    durability: i32,
    damage: Option<i32>,
    healing: Option<i32>,
}

fn get_item_description(item_type: &ItemType) -> ItemDescriptor {
    match item_type {
        ItemType::Rifle => ItemDescriptor {
            durability: 5,
            damage: Some(30),
            healing: None,
        },
        ItemType::SmallMed => ItemDescriptor {
            durability: 1,
            damage: None,
            healing: Some(20),
        },
        ItemType::BigMed => ItemDescriptor {
            durability: 1,
            damage: None,
            healing: Some(50),
        },
        ItemType::Sword => ItemDescriptor {
            durability: 10,
            damage: Some(15),
            healing: None,
        },
        ItemType::Shotgun => ItemDescriptor {
            durability: 2,
            damage: Some(50),
            healing: None,
        },
    }
}

struct Node {
    item_type: ItemType,
}

impl Node {
    fn new(item_type: ItemType) -> Self {
        Self { item_type }
    }

    fn render(&self) {
        let descriptor = get_item_description(&self.item_type);
        println!("------------------------------");
        println!("Item: {}", self.item_type.string());
        println!("Durability: {}", descriptor.durability);
        match descriptor.damage {
            Some(damage) => println!("Damage: {}", damage),
            None => println!("Damage: None"),
        }
        match descriptor.healing {
            Some(healing) => println!("Healing: {}", healing),
            None => println!("Healing: None"),
        }
        println!("------------------------------");
    }
}

//(NOTE): if we have two words with the same prefix (eg. map and maple) the shorter word will win
//for example with command maple the map will register and not the maple

struct WordProgress {
    word: String,
    current_char: Option<char>,
    current_word_index: usize,
    word_size: usize,
    freeze: bool,
    started: bool,
    return_type: InvCommand,
}

impl WordProgress {
    pub fn new(word: String, return_type: InvCommand) -> Self {
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

    pub fn check_char(&mut self, c: &char, freezed_words: &mut usize) -> Option<Message> {
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

    fn get_return_type(&self) -> InvCommand {
        self.return_type
    }

    fn size(&self) -> usize{
        self.word_size
    }
}

enum CharacterValidation {
    CorrectCharacter,
    IncorrectCharacter,
    NoCharacter,
}

#[derive(PartialEq)]
enum Message {
    StartedWord,
    FinishedWord,
    IncorrectWord,
    WordTypedIncorrectly,
    GameTutorial,
}

#[derive(PartialEq, Copy, Clone)]
enum InvCommand {
    Map,
    Quit,
    Equip,
    Drop,
    Select(usize),
    Tutorial,
}

pub struct InventoryManager {
    nodes: Vec<Node>,
    searched_words: Vec<WordProgress>,
    disable_node_rendering_flag: bool,
    active_words: usize, //abstract it to input parser manager
    freezed_words: usize,
}

impl InventoryManager {
    pub fn new(disable_node_rendering_flag: bool) -> Self {
        let map_progress = WordProgress::new("map".to_string(), InvCommand::Map);
        let quit_progress = WordProgress::new("quit".to_string(), InvCommand::Quit);
        let equip_progress = WordProgress::new("equip".to_string(), InvCommand::Equip);
        let drop_progress = WordProgress::new("drop".to_string(), InvCommand::Drop);
        let tutorial_progress = WordProgress::new("info".to_string(), InvCommand::Tutorial);

        let mut searched_words = Vec::new();
        searched_words.push(map_progress);
        searched_words.push(quit_progress);
        searched_words.push(equip_progress);
        searched_words.push(drop_progress);
        searched_words.push(tutorial_progress);

        Self {
            nodes: Vec::new(),
            searched_words,
            disable_node_rendering_flag,
            active_words: 0,
            freezed_words: 0,
        }
    }

    pub fn add_node(&mut self, item_type: ItemType) {
        self.nodes.push(Node::new(item_type));
        let length = self.nodes.len();
        self.searched_words.push(WordProgress::new(
            length.to_string(),
            InvCommand::Select(length),
        ));
    }

    pub fn render(&mut self) {
        if self.disable_node_rendering_flag {
            return;
        }
        for node in self.nodes.iter() {
            node.render();
        }
    }

    pub fn update(&mut self, location_type: &mut LocationType) -> bool {
        let mut line = String::new();
        let _byte_size = std::io::stdin().read_line(&mut line).unwrap();
        let command: String = line.chars().filter(|c| !c.is_whitespace()).collect();
        let mut queue: data_structures::Queue<InvCommand> = data_structures::Queue::new();

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
                    Some(Message::WordTypedIncorrectly) => println!("You've made a mistake"),
                    _ => (),
                }
            }
            //println!("freezed words {} searched words {}", self.freezed_words, self.searched_words.len());
            if self.freezed_words == self.searched_words.len(){
                println!("bad");
            }

            //reseting every wordProgress when one word is finished
            if msg == Some(Message::FinishedWord) {
                for searched_word in self.searched_words.iter_mut() {
                    searched_word.reset();
                }
                self.freezed_words = 0;
            }
        }

        for _ in 0..queue.size() {
            let command = queue.pop();
            match command {
                InvCommand::Map => {
                    *location_type = LocationType::Map;
                    println!("Going to map");
                }
                InvCommand::Quit => println!("Quitting"),
                InvCommand::Drop => println!("Dropping"),
                InvCommand::Equip => println!("Equppin"),
                InvCommand::Select(item_number) => {
                    println!("Selecting number {}", item_number);
                }
                InvCommand::Tutorial => {
                    println!("Tutorial");
                }
            }
        }
        true
    }
}
