use std::io::{self, BufRead};
use crate::ItemType;

struct Node {
    item_type: ItemType,
}

impl Node {
    fn new(item_type: ItemType) -> Self {
        Self {
            item_type
        }
    }
    fn render(&self) {
        match self.item_type{
            ItemType::Rifle => {
                println!("--------------------");
                println!("       AK-47");
                println!("--------------------");
            },
            ItemType::SmallMed => {
                println!("--------------------");
                println!("       smol med");
                println!("--------------------");
            },
            ItemType::BigMed => {
                println!("--------------------");
                println!("       big med");
                println!("--------------------");
            },
            ItemType::Sword => {
                println!("--------------------");
                println!("       Sowrd");
                println!("--------------------");
            },
            ItemType::Shotgun => {
                println!("--------------------");
                println!("       shotty");
                println!("--------------------");
            },

        }
    }
}

pub struct InventoryManager {
    nodes: Vec<Node>,
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

    pub fn update(&self) -> bool {
        let stdin = io::stdin();
        let input = stdin.lock().lines().next().unwrap().unwrap();
        true
    }
}
