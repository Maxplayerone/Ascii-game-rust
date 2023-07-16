use crate::{
    chest, data_structures, enemy, inventory, level_parser, math, parser, player, LocationType,
};

const ENEMY_SYMBOL: char = '@';
const PLAYER_SYMBOL: char = '0';
const GRASS_SYMBOL: char = 'x';
const CHEST_SYMBOL: char = '=';

pub struct MapManager {
    map: Vec<char>,
    map_dimensions: math::Pos2,

    parser: parser::ParserManager<MapCommand>,

    enemy_manager: enemy::EnemyManager,
    chests: Vec<chest::Chest>,
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
    pub fn new() -> (Self, math::Pos2) {
        let (map, map_info) = level_parser::parse_level(PLAYER_SYMBOL, ENEMY_SYMBOL, CHEST_SYMBOL);
        let map_dimensions = map_info.map_dimensions;
        let enemy_manager = enemy::EnemyManager::new(map_info.enemies);
        let chests = map_info.chests;
        let player_pos = map_info.player;

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
        (
            Self {
                map,
                map_dimensions,
                parser,
                enemy_manager,
                chests,
            },
            player_pos,
        )
    }
    // (NOTE) we have inventory_manager in the arguments because we give it the items via input_parser
    pub fn update(
        &mut self,
        location_type: &mut LocationType,
        player: &mut player::PlayerManager,
    ) -> bool {
        let queue: Option<data_structures::Queue<MapCommand>> = self.parser.parse();
        let mut move_multiplier = 1;
        match queue {
            Some(mut queue) => {
                for _ in 0..queue.size() {
                    let command = queue.pop();
                    match command {
                        MapCommand::Inv => {
                            *location_type = LocationType::Inventory;
                        }
                        MapCommand::Info => {
                            println!("showing the tutorial...");
                        }
                        MapCommand::Left
                        | MapCommand::Right
                        | MapCommand::Up
                        | MapCommand::Down => {
                            for _ in 0..move_multiplier {
                                player.update(command);
                                //player + chest collission detection + resolution
                                let size = self.chests.len();
                                for i in 0..size {
                                    if are_colliding(&self.chests[i].pos, &player.pos) {
                                        player.add_item(*self.chests[i].get_item());
                                        player.set_new_item_bool(true);
                                        //we're moving the value from chests cuz we're deleting
                                        //the chest thingy down there
                                        if size > 1 {
                                            self.chests[size - 1] = self.chests[i];
                                            self.chests.pop();
                                        } else {
                                            self.chests.pop();
                                        }
                                        println!("Colliding");
                                    }
                                }

                                self.enemy_manager.update(&player.pos);
                            }
                            move_multiplier = 1;
                        }
                        MapCommand::Wait => {
                            for _ in 0..move_multiplier {
                                self.enemy_manager.update(&player.pos);
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
        true
    }

    fn flush(&mut self) {
        self.map.iter_mut().for_each(|c| *c = GRASS_SYMBOL);

        let width_usize: usize = self.map_dimensions.x.try_into().unwrap();
        for i in 0..self.map_dimensions.y {
            self.map[i as usize * (width_usize + 1) + width_usize] = '\n';
        }
    }

    pub fn render(&mut self, player: &player::PlayerManager) {
        //flusing the map from the last game tick
        self.flush();

        //rendering player
        let width_usize: usize = self.map_dimensions.x.try_into().unwrap();
        let (x, y) = player.get_position();
        self.map[y * width_usize + x] = PLAYER_SYMBOL;

        //rendering enemies
        for i in 0..self.enemy_manager.size() {
            let (x, y) = self.enemy_manager.get_enemy_position(i);
            self.map[y * width_usize + x] = ENEMY_SYMBOL;
        }

        //rendering chests
        for i in 0..self.chests.len() {
            let x: usize = self.chests[i].pos.x.try_into().unwrap();
            let y: usize = self.chests[i].pos.y.try_into().unwrap();
            self.map[y * width_usize + x] = CHEST_SYMBOL;
        }

        //rendering the rest
        let map_string: String = self.map.iter().collect();
        println!("{}", map_string);
    }
}

fn are_colliding(pos1: &math::Pos2, pos2: &math::Pos2) -> bool {
    pos1.x == pos2.x && pos1.y == pos2.y
}
