use crate::{map, math, weapons};

pub struct PlayerManager {
    pub pos: math::Pos2,
    items: Vec<weapons::ItemType>,
    new_item: bool,
}

impl PlayerManager {
    pub fn new(pos: math::Pos2) -> Self {
        Self { pos, items: Vec::new(), new_item: false}
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

    pub fn add_item(&mut self, item: weapons::ItemType){
        self.items.push(item);
    }

    pub fn get_most_recent_item(&mut self) -> weapons::ItemType{
        self.set_new_item_bool(false);
        self.items[self.items.len() - 1]
    }

    pub fn set_new_item_bool(&mut self, thing: bool){
        self.new_item = thing;
    }

    pub fn get_new_item_bool(&self) -> bool{
        self.new_item
    }
}
