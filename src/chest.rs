use crate::{weapons, math};

#[derive(Copy, Clone)]
pub struct Chest{
    pub pos: math::Pos2,
    item_type: weapons::ItemType,
}

impl Chest{
    pub fn new(pos: math::Pos2, item_type: weapons::ItemType) -> Self{
        Self{
            pos,
            item_type,
        }
    }
    pub fn get_item(&self) -> &weapons::ItemType{
        &self.item_type
    }
}


