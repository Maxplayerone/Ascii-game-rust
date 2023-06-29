use crate::{input_parser, math};

pub struct PlayerManager{
    pub pos: math::Pos2,
}

impl PlayerManager{
    pub fn new(pos: math::Pos2) -> Self{
        Self{
            pos
        }
    }
    
    pub fn update(&mut self, input_command: input_parser::InputCommand){
        match input_command {
            input_parser::InputCommand::Right => self.pos.x += 1,
            input_parser::InputCommand::Left => self.pos.x -= 1,
            input_parser::InputCommand::Up => self.pos.y -= 1,
            input_parser::InputCommand::Down => self.pos.y += 1,
            _ => {
                ()
            }
        }
    }
    
    pub fn get_position(&self) -> (usize, usize){
        let x: usize = self.pos.x.try_into().unwrap();
        let y: usize = self.pos.y.try_into().unwrap();
        (x, y)
    }
}
