mod data_structures;
mod input_parser;
mod math;
mod enemy;
mod level_parser;

//(NOTE): how coordinate system works
//(0, 0) top-left
//const CHUNK_WIDTH: usize = 32;
//we have to add one more for the newline
//const CHUNK_WIDTH_NEWLINE: usize = CHUNK_WIDTH + 1;
//const CHUNK_HEIGHT: usize = 30;

struct GameState {
    map: Vec<char>,
    map_dimensions: math::Pos2,
    player_pos: math::Pos2,
    enemy_manager: enemy::EnemyManager,
}

const ENEMY_SYMBOL: char = '@';
const PLAYER_SYMBOL: char = '0';
const GRASS_SYMBOL: char = 'x';

impl GameState {
    fn new() -> Self {
        let (map, info) = level_parser::parse_level(PLAYER_SYMBOL, ENEMY_SYMBOL);
        let player_pos = info.player;
        let map_dimensions = info.map_dimensions;
        let enemy_manager = enemy::EnemyManager::new(info.enemies);
        Self {
            map,
            player_pos,
            map_dimensions,
            enemy_manager,
        }
    }

    fn update_enemies(&mut self) {
        self.enemy_manager.update_enemies(&self.player_pos);
    }

    fn render_enemies(&mut self) {
        let width_usize: usize = self.map_dimensions.x.try_into().unwrap();

        for i in 0..self.enemy_manager.size(){
            let enemy = self.enemy_manager.get_enemy(i);

            let y: usize = enemy.y.try_into().unwrap();
            let x: usize = enemy.x.try_into().unwrap();

            self.map[y * width_usize + x] = ENEMY_SYMBOL;
        }
    }

    fn update_hero(&mut self, input_command: input_parser::InputCommand) {
        match input_command {
            input_parser::InputCommand::Right => self.player_pos.x += 1,
            input_parser::InputCommand::Left => self.player_pos.x -= 1,
            input_parser::InputCommand::Up => self.player_pos.y -= 1,
            input_parser::InputCommand::Down => self.player_pos.y += 1,
            _ => {
                let _ = 3;
            }
        }
    }

    fn render_hero(&mut self) {
        let width_usize: usize = self.map_dimensions.x.try_into().unwrap();
        let y: usize = self.player_pos.y.try_into().unwrap();
        let x: usize = self.player_pos.x.try_into().unwrap();
        self.map[y * width_usize + x] = PLAYER_SYMBOL;
    }

    fn update_map(&mut self) -> bool {
        let mut queue = input_parser::get_parsed_user_input_map();

        for _ in 0..queue.size() {
            let command: input_parser::InputCommand = queue.pop();
            if command == input_parser::InputCommand::Quit {
                return false;
            } else {
                self.update_hero(command);
            }

            self.update_enemies();
        }
        true
    }

    fn flush_map(&mut self) {
        self.map.iter_mut().for_each(|c| *c = 'x');

        let width_usize: usize = self.map_dimensions.x.try_into().unwrap();
        let mut index: usize;
        for i in 0..self.map_dimensions.y {
            self.map[i as usize * (width_usize + 1) + width_usize] = '\n';
        }  
    }

    fn render_map(&mut self) {
        self.flush_map();
        self.render_hero();
        self.render_enemies();

        let map_string: String = self.map.iter().collect();
        println!("{}", map_string);
    }
}

fn main() {
    let mut state = GameState::new();
    let mut playing = true;
    while playing {
        state.render_map();
        playing = state.update_map();
    }
}
