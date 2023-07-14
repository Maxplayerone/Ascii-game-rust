use crate::{data_structures, enemy, inventory, level_parser, math, parser, player, LocationType};

pub struct MapManager {
    map: Vec<char>,
    map_dimensions: math::Pos2,

    player_symbol: char,
    enemy_symbol: char,
    grass_symbol: char,

    parser: parser::ParserManager<MapCommand>,

    player_manager: player::PlayerManager,
    enemy_manager: enemy::EnemyManager,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MapCommand {
    Right,
    Left,
    Up,
    Down,
    Wait,
    Quit,
    Inv,
    Info,
    MoveCount(usize),
}

impl MapManager {
    pub fn new(player_symbol: char, enemy_symbol: char, grass_symbol: char) -> Self {
        let (map, map_info) = level_parser::parse_level(player_symbol, enemy_symbol);
        let map_dimensions = map_info.map_dimensions;
        let player_manager = player::PlayerManager::new(map_info.player);
        let enemy_manager = enemy::EnemyManager::new(map_info.enemies);

        let left = parser::WordProgress::new("left".to_string(), MapCommand::Left);
        let right = parser::WordProgress::new("right".to_string(), MapCommand::Right);
        let down = parser::WordProgress::new("down".to_string(), MapCommand::Down);
        let up = parser::WordProgress::new("up".to_string(), MapCommand::Up);
        let wait = parser::WordProgress::new("wait".to_string(), MapCommand::Wait);
        let quit = parser::WordProgress::new("quit".to_string(), MapCommand::Quit);
        let inv = parser::WordProgress::new("inv".to_string(), MapCommand::Inv);
        let info = parser::WordProgress::new("tut".to_string(), MapCommand::Info);

        let mut searched_words = Vec::new();
        searched_words.push(left);
        searched_words.push(right);
        searched_words.push(up);
        searched_words.push(down);
        searched_words.push(wait);
        searched_words.push(quit);
        searched_words.push(inv);
        searched_words.push(info);
        for i in 0..10 {
            searched_words.push(parser::WordProgress::new(
                i.to_string(),
                MapCommand::MoveCount(i),
            ));
        }

        let parser = parser::ParserManager::<MapCommand>::new(searched_words);
        Self {
            player_symbol,
            enemy_symbol,
            grass_symbol,
            map,
            map_dimensions,
            parser,
            player_manager,
            enemy_manager,
        }
    }
    // (NOTE) we have inventory_manager in the arguments because we give it the items via input_parser
    pub fn update(&mut self, location_type: &mut LocationType) -> bool {
        let queue: Option<data_structures::Queue<MapCommand>> = self.parser.parse();
        let mut move_multiplier = 1;
        match queue {
            Some(mut queue) => {
                for _ in 0..queue.size() {
                    let command = queue.pop();
                    match command {
                        MapCommand::Inv => {
                            println!("Inventory time");
                            *location_type = LocationType::Inventory;
                        }
                        MapCommand::Info => {
                            println!("showing the tutorial...");
                        }
                        MapCommand::Left
                        | MapCommand::Right
                        | MapCommand::Up
                        | MapCommand::Down => {
                            for _ in 0..move_multiplier{
                                self.player_manager.update(command);
                                self.enemy_manager.update(&self.player_manager.pos);
                            }
                            move_multiplier = 1;
                        }
                        MapCommand::Wait => {
                            for _ in 0..move_multiplier{
                                self.enemy_manager.update(&self.player_manager.pos);
                            }
                            move_multiplier = 1;
                        }
                        MapCommand::MoveCount(count) => {
                            move_multiplier = count;
                        }
                        MapCommand::Quit => {
                            return false;
                        }
                    }
                }
            }
            None => self.parser.reset(),
        }
        /*
        let (mut queue, location_changer) =
            input_parser::get_parsed_user_input_map(inventory_manager);
        if location_changer.is_some() {
            *location_type = LocationType::Inventory;
        }

        for _ in 0..queue.size() {
            let command: input_parser::InputCommand = queue.pop();
            if command == input_parser::InputCommand::Quit {
                return false;
            } else {
                player_manager.update(command);
            }
            enemy_manager.update(&player_manager.pos);
        }
        */
        true
    }

    fn flush(&mut self) {
        self.map.iter_mut().for_each(|c| *c = self.grass_symbol);

        let width_usize: usize = self.map_dimensions.x.try_into().unwrap();
        for i in 0..self.map_dimensions.y {
            self.map[i as usize * (width_usize + 1) + width_usize] = '\n';
        }
    }

    pub fn render(&mut self) {
        //flusing the map from the last game tick
        self.flush();
        //rendering player
        let width_usize: usize = self.map_dimensions.x.try_into().unwrap();
        let (x, y) = self.player_manager.get_position();
        self.map[y * width_usize + x] = self.player_symbol;
        //rendering enemies
        for i in 0..self.enemy_manager.size() {
            let (x, y) = self.enemy_manager.get_enemy_position(i);
            self.map[y * width_usize + x] = self.enemy_symbol;
        }

        //rendering the rest
        let map_string: String = self.map.iter().collect();
        println!("{}", map_string);
    }
}
