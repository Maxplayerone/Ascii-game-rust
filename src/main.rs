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
    player_pos: math::Pos2,
    enemy_manager: enemy::EnemyManager,
}

const ENEMY_SYMBOL: char = '@';
const PLAYER_SYMBOL: char = '0';
const GRASS_SYMBOL: char = 'x';

impl GameState {
    fn new() -> Self {
        let enemy_manager = enemy::EnemyManager::new(ENEMY_SYMBOL, 1);
        let map = level_parser::parse_level();
        Self {
            map,
            player_pos: math::Pos2::new(0, 0),
            enemy_manager,
        }
    }

    fn add_enemy(&mut self, pos: math::Pos2) -> usize {
        self.enemy_manager.add_enemy(pos)
    }

    fn update_enemies(&mut self) {
        self.enemy_manager.update_enemies(&self.player_pos);
    }

    fn render_enemies(&mut self) {
        /*
        for i in 0..self.enemy_manager.size(){
            let enemy = self.enemy_manager.get_enemy(i);
            self.map_repr[enemy.y as usize][enemy.x as usize] = ENEMY_SYMBOL;
        }
        */
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
        //self.map_repr[self.player_pos.y as usize][self.player_pos.x as usize] = PLAYER_SYMBOL;
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
        /*
        for i in 0..CHUNK_WIDTH_NEWLINE {
            for j in 0..CHUNK_HEIGHT {
                self.map_repr[j][i] = GRASS_SYMBOL;
            }
        }

        for i in 0..CHUNK_HEIGHT {
            self.map_repr[i][CHUNK_WIDTH] = '\n';
        }
        */
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
    let enemy_index = state.add_enemy(math::Pos2::new(5, 7));
    let mut playing = true;
    while playing {
        state.render_map();
        playing = state.update_map();
    }
}
