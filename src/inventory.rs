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

pub struct InventoryManager {
    nodes: Vec<Node>,
    searched_words: Vec<WordProgress>,
}

//(NOTE): if we have two words with the same prefix (eg. map and maple) the shorter word will win
//for example with command maple the map will register and not the maple
struct WordProgress {
    word: String,
    current_char: Option<char>,
    current_word_index: usize,
    word_size: usize,
    started_word: bool,
    freeze: bool,
}

impl WordProgress {
    pub fn new(word: String) -> Self {
        let word = word.clone();
        let word_size = word.chars().count();
        Self {
            word,
            current_char: None,
            current_word_index: 0,
            word_size,
            started_word: false,
            freeze: false,
        }
    }

    fn get_char_from_index(&mut self, index: usize) -> &mut Self {
        self.current_char = self.word.chars().nth(index);
        self
    }

    fn get_char_from_index_bool(&mut self, index: usize) -> bool{
        self.word.chars().nth(index).is_some()
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

    pub fn check_char(&mut self, c: &char) -> Option<WordProgressMessage> {
        if self.freeze {
            return None;
        }

        match self
            .get_char_from_index(self.current_word_index)
            .equate_chars(c)
        {
            CharacterValidation::CorrectCharacter => {
                self.current_word_index += 1;

                if self.current_word_index == self.word_size {
                    println!("Finished the word {}", self.word);
                    return Some(WordProgressMessage::FinishedWord);
                }

                if !self.started_word {
                    self.started_word = true;
                    println!("Started the word {}", self.word);
                    return Some(WordProgressMessage::StartedWord);
                }
            }
            CharacterValidation::IncorrectCharacter => {
                println!("The word {} is not correct", self.word);
                self.freeze = true;
            }
            CharacterValidation::NoCharacter => {
                println!("Finished a word ({})", self.word);
                return Some(WordProgressMessage::FinishedWord);
            }
        }
        None
    }

    fn reset(&mut self) {
        self.current_word_index = 0;
        self.freeze = false;
        self.started_word = false;
    }
}

enum CharacterValidation {
    CorrectCharacter,
    IncorrectCharacter,
    NoCharacter,
}

#[derive(PartialEq)]
enum WordProgressMessage {
    StartedWord,
    FinishedWord,
}

impl InventoryManager {
    pub fn new() -> Self {
        let map_progress = WordProgress::new("map".to_string());
        let quit_progress = WordProgress::new("quit".to_string());
        let equip_progress = WordProgress::new("equip".to_string());
        let info_progress = WordProgress::new("info".to_string());
        let inv_progress = WordProgress::new("inv".to_string());
        /*
        let drop_progress = WordProgress::new("drop".to_string());
        */

        let mut searched_words = Vec::new();
        searched_words.push(map_progress);
        searched_words.push(quit_progress);
        searched_words.push(equip_progress);
        searched_words.push(info_progress);
        searched_words.push(inv_progress);
        /*
        searched_words.push(drop_progress);
        */

        Self {
            nodes: Vec::new(),
            searched_words,
        }
    }

    pub fn add_node(&mut self, item_type: ItemType) {
        self.nodes.push(Node::new(item_type));
        //self.searched_words.push(WordProgress::new(self.nodes.len().to_string()));
    }

    pub fn render(&mut self) {
        for node in self.nodes.iter() {
            node.render();
        }
    }

    pub fn update(&mut self, location_type: &mut LocationType) -> bool {
        let mut line = String::new();
        let _byte_size = std::io::stdin().read_line(&mut line).unwrap();
        let command: String = line.chars().filter(|c| !c.is_whitespace()).collect();

        //when adding helpful messages try to use iterator patterns
        /*
        if equate_chars(line.chars().next(), 'm')
            && equate_chars(line.chars().nth(1), 'a')
            && equate_chars(line.chars().nth(2), 'p')
            && equate_chars(line.chars().nth(3), 13 as char)
        {
            *location_type = LocationType::Map;
        }
        */

        for c in command.chars() {
            let mut msg: Option<WordProgressMessage> = None;
            for searched_word in self.searched_words.iter_mut() {
                let return_val = searched_word.check_char(&c);

                if return_val.is_some(){
                    msg = return_val;
                }
            }

            if msg == Some(WordProgressMessage::FinishedWord) {
                for searched_word in self.searched_words.iter_mut() {
                        searched_word.reset();
                }
            }
        }

        true
    }
}
