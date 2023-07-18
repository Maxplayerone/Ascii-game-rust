use crate::{map, math, weapons};

pub struct PlayerManager {
    pub pos: math::Pos2,

    pub items: Vec<weapons::Item>,
    pub got_new_item: bool,

    health: usize,
    name: String,

    pub current_selected_item: Option<usize>,
}

impl PlayerManager {
    pub fn new(pos: math::Pos2, name: String, health: usize) -> Self {
        Self {
            pos,
            name,
            health,
            items: Vec::new(),
            got_new_item: false,
            current_selected_item: None,
        }
    }

    pub fn update(&mut self, input_command: map::MapCommand) {
        match input_command {
            map::MapCommand::Right => self.pos.x += 1,
            map::MapCommand::Left => self.pos.x -= 1,
            map::MapCommand::Up => self.pos.y -= 1,
            map::MapCommand::Down => self.pos.y += 1,
            _ => (),
        }
    }

    pub fn get_position(&self) -> (usize, usize) {
        let x: usize = self.pos.x.try_into().unwrap();
        let y: usize = self.pos.y.try_into().unwrap();
        (x, y)
    }

    pub fn add_item(&mut self, item_type: weapons::ItemType) {
        self.items.push(weapons::Item::new(item_type));
        self.got_new_item = true;
    }

    pub fn item_count(&mut self) -> usize {
        self.items.len()
    }

    pub fn remove_item(&mut self, index: usize) {
        let vec_size = self.item_count();
        self.items[index] = self.items[vec_size - 1];
        self.items.pop();
    }

    pub fn set_got_new_item(&mut self, did_got_new_item: bool) {
        self.got_new_item = did_got_new_item
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_health(&self) -> usize {
        self.health
    }

    pub fn get_item_count(&self) -> usize {
        self.items.len()
    }
}
