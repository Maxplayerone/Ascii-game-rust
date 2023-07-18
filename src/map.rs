use crate::{chest, data_structures, enemy, level_parser, math, parser, player, LocationType};

const ENEMY_SYMBOL: char = '@';
const PLAYER_SYMBOL: char = '0';
const GRASS_SYMBOL: char = 'x';
const CHEST_SYMBOL: char = '=';
const UNBREAKABLE_SYMBOL: char = '&';

pub struct MapManager {
    map: Vec<char>,
    map_dimensions: math::Pos2,

    parser: parser::ParserManager<MapCommand>,

    enemy_manager: enemy::EnemyManager,
    chests: Vec<chest::Chest>,
    unbreakable: Vec<math::Pos2>,
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
        let (map, map_info) = level_parser::parse_level(
            PLAYER_SYMBOL,
            ENEMY_SYMBOL,
            CHEST_SYMBOL,
            UNBREAKABLE_SYMBOL,
        );
        let map_dimensions = map_info.map_dimensions;
        let enemy_manager = enemy::EnemyManager::new(map_info.enemies);
        let chests = map_info.chests;
        let player_pos = map_info.player;
        let unbreakable = map_info.unbreakable;

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
                chests,
                unbreakable,
                enemy_manager,
            },
            player_pos,
        )
    }

    fn check_if_player_can_move(&self, command: &MapCommand, player_pos: (usize, usize)) -> bool {
        let mut x = player_pos.0.try_into().unwrap();
        let mut y = player_pos.1.try_into().unwrap();
        match command {
            MapCommand::Left => {
                x -= 1
            }
            MapCommand::Right => {
                x += 1
            }
            MapCommand::Up => {
                y += 1
            }
            MapCommand::Down => {
                y -= 1
            }
            _ => return false,
        }

        for block in self.unbreakable.iter() {
            if block.x == x && block.y == y {
                println!("Cannot move");
                return false;
            }
        }
        true
    }

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
                                if self.check_if_player_can_move(&command, player.get_position()) {
                                    player.update(command);
                                    //player + chest collission detection + resolution
                                    let size = self.chests.len();
                                    for i in 0..size {
                                        if are_colliding(&self.chests[i].pos, &player.pos) {
                                            println!("heeeeeeelo");
                                            player.add_item(*self.chests[i].get_item());
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

                                    self.enemy_manager.update(&player.pos, &self.unbreakable, location_type);
                                }
                            }
                            move_multiplier = 1;
                        }
                        MapCommand::Wait => {
                            for _ in 0..move_multiplier {
                                self.enemy_manager.update(&player.pos, &self.unbreakable, location_type);
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

        //rendering unbreakable blocks
        for i in 0..self.unbreakable.len() {
            let x: usize = self.unbreakable[i].x.try_into().unwrap();
            let y: usize = self.unbreakable[i].y.try_into().unwrap();
            self.map[y * width_usize + x] = UNBREAKABLE_SYMBOL;
        }

        //rendering the rest
        let map_string: String = self.map.iter().collect();
        println!("{}", map_string);
    }
}

fn are_colliding(pos1: &math::Pos2, pos2: &math::Pos2) -> bool {
    pos1.x == pos2.x && pos1.y == pos2.y
}
