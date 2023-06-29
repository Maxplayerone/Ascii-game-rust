use crate::{LocationType, enemy, player, level_parser, input_parser, inventory, math};

pub struct MapManager{
    map: Vec<char>,
    map_dimensions: math::Pos2,
    player_symbol: char,
    enemy_symbol: char,
}

impl MapManager{
    pub fn new(player_symbol: char, enemy_symbol: char) -> (Self, level_parser::ParserInfo){
        let (map, info) = level_parser::parse_level(player_symbol, enemy_symbol);
        let map_dimensions = info.map_dimensions;
        (
        Self{
            map,
            map_dimensions,
            player_symbol,
            enemy_symbol,
        },
        info)
    }
    // (NOTE) we have inventory_manager in the arguments because we give it the items via input_parser
    pub fn update(&mut self, 
        inventory_manager: &mut inventory::InventoryManager, 
        location_type: &mut LocationType,
        enemy_manager: &mut enemy::EnemyManager,
        player_manager: &mut player::PlayerManager
        ) -> bool{
        let (mut queue, location_changer) = input_parser::get_parsed_user_input_map(inventory_manager);
        if let Some(location_changer) = location_changer{
            *location_type = LocationType::Inventory;
        }
        
        for _ in 0..queue.size(){
            let command: input_parser::InputCommand = queue.pop();
            if command == input_parser::InputCommand::Quit{
                return false;
            }
            else{
                player_manager.update(command);
            }
            enemy_manager.update(&player_manager.pos);
        }
        true
    }
    
    fn flush(&mut self){
        self.map.iter_mut().for_each(|c| *c = 'x');

        let width_usize: usize = self.map_dimensions.x.try_into().unwrap();
        let mut index: usize;
        for i in 0..self.map_dimensions.y {
            self.map[i as usize * (width_usize + 1) + width_usize] = '\n';
        }
    }
    
    pub fn render(&mut self,
                    player_manager: &mut player::PlayerManager,
                    enemy_manager: &mut enemy::EnemyManager){
        //flusing the map from the last game tick
        self.flush();
        //rendering player
        let width_usize: usize = self.map_dimensions.x.try_into().unwrap();
        let (x, y) = player_manager.get_position();
        self.map[y * width_usize + x] = self.player_symbol;
        //rendering enemies
        for i in 0..enemy_manager.size(){
            let (x, y) = enemy_manager.get_enemy_position(i);
            self.map[y * width_usize + x] = self.enemy_symbol;
        }
        
        //rendering the rest
        let map_string: String = self.map.iter().collect();
        println!("{}", map_string);
    }
}
