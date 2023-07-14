use crate::data_structures;
use crate::parser;
use crate::LocationType;
use crate::weapons;

struct ItemDescriptor {
    durability: i32,
    damage: Option<i32>,
    healing: Option<i32>,
}

fn get_item_description(item_type: &weapons::ItemType) -> ItemDescriptor {
    match item_type {
        weapons::ItemType::Rifle => ItemDescriptor {
            durability: 5,
            damage: Some(30),
            healing: None,
        },
        weapons::ItemType::SmallMed => ItemDescriptor {
            durability: 1,
            damage: None,
            healing: Some(20),
        },
        weapons::ItemType::BigMed => ItemDescriptor {
            durability: 1,
            damage: None,
            healing: Some(50),
        },
        weapons::ItemType::Sword => ItemDescriptor {
            durability: 10,
            damage: Some(15),
            healing: None,
        },
        weapons::ItemType::Shotgun => ItemDescriptor {
            durability: 2,
            damage: Some(50),
            healing: None,
        },
    }
}

struct Node {
    item_type: weapons::ItemType,
}

impl Node {
    fn new(item_type: weapons::ItemType) -> Self {
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
    disable_node_rendering_flag: bool,
    parser: parser::ParserManager<InvCommand>,
}

impl InventoryManager {
    pub fn new(disable_node_rendering_flag: bool) -> Self {
        let map_progress = parser::WordProgress::new("map".to_string(), InvCommand::Map);
        let quit_progress = parser::WordProgress::new("quit".to_string(), InvCommand::Quit);
        let equip_progress = parser::WordProgress::new("equip".to_string(), InvCommand::Equip);
        let drop_progress = parser::WordProgress::new("drop".to_string(), InvCommand::Drop);
        let tutorial_progress = parser::WordProgress::new("info".to_string(), InvCommand::Tutorial);

        let mut searched_words = Vec::new();
        searched_words.push(map_progress);
        searched_words.push(quit_progress);
        searched_words.push(equip_progress);
        searched_words.push(drop_progress);
        searched_words.push(tutorial_progress);
        let parser = parser::ParserManager::<InvCommand>::new(searched_words);

        Self {
            disable_node_rendering_flag,
            nodes: Vec::new(),
            parser,
        }
    }

    pub fn add_node(&mut self, item_type: weapons::ItemType) {
        self.nodes.push(Node::new(item_type));

        let length = self.nodes.len();

        self.parser.add_word(parser::WordProgress::new(
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
        let queue: Option<data_structures::Queue<InvCommand>> = self.parser.parse();

        match queue {
            Some(mut queue) => {
                for _ in 0..queue.size() {
                    let command = queue.pop();
                    match command {
                        InvCommand::Map => {
                            *location_type = LocationType::Map;
                            println!("Going to map");
                        }
                        InvCommand::Quit => return false,
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
            }
            None => (),
        }
        true
    }
}
