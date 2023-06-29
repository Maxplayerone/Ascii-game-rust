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
}

fn equate_chars(char_option: Option<char>, char_to_compare: char) -> bool {
    match char_option {
        Some(c) => {
            if c == char_to_compare {
                return true;
            } else {
                return false;
            }
        }
        None => false,
    }
}

impl InventoryManager {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, item_type: ItemType) {
        self.nodes.push(Node::new(item_type));
    }

    pub fn render(&mut self) {
        for node in self.nodes.iter() {
            node.render();
        }
    }

    pub fn update(&self, location_type: &mut LocationType) -> bool {
        let mut line = String::new();
        let _byte_size = std::io::stdin().read_line(&mut line).unwrap();

        if equate_chars(line.chars().nth(0), 'm')
            && equate_chars(line.chars().nth(1), 'a')
            && equate_chars(line.chars().nth(2), 'p')
            && equate_chars(line.chars().nth(3), 13 as char)
        {
            *location_type = LocationType::Map;
        }

        true
    }
}
