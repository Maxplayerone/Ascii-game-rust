use arrayvec::ArrayVec;

mod data_structures;
mod input_parser;
mod math;

//(NOTE): how coordinate system works
//(0, 0) top-left
const CHUNK_WIDTH: usize = 32;
//we have to add one more for the newline
const CHUNK_WIDTH_NEWLINE: usize = CHUNK_WIDTH + 1;
const CHUNK_HEIGHT: usize = 30;

struct GameState {
    map_repr: [[char; CHUNK_WIDTH_NEWLINE]; CHUNK_HEIGHT],
    player_pos: math::Pos2,
    enemies: Vec<math::Pos2>,
    enemy_index: usize,
}

const ENEMY_SYMBOL: char = '@';
const PLAYER_SYMBOL: char = '0';
const GRASS_SYMBOL: char = 'x';

fn find_closest_position_to_player(enemy: &math::Pos2, player: &math::Pos2) -> math::Pos2 {
    //println!("Player coordinates: {} | {} \n Enemy coordinates: {} | {}", player.x, player.y, enemy.x, enemy.y);
    let diff_x = (enemy.x - player.x).abs();
    let diff_y = (enemy.y - player.y).abs();
    //println!("Diff_x {} diff_y {}", diff_x, diff_y);

    //SPECIAL CASE WHEN ENEMY SPAWNS AT PLAYER POSITION
    if diff_x == 0 && diff_y == 0 {
        assert!(false, "Enemy spawned at player position");
        return math::Pos2::new(0, 0);
    }

    //move on the y_xis
    if diff_y > diff_x {
        if player.y > enemy.y {
            let offset = math::Pos2::new(0, 1);
            let pos_with_offset = math::Pos2::new(enemy.x + offset.x, enemy.y + offset.y);

            if player.x == pos_with_offset.x && player.y == pos_with_offset.y {
                assert!(false, "Enemy caught the player");
                math::Pos2::new(0, 0)
            } else {
                offset
            }
        } else {
            let offset = math::Pos2::new(0, -1);
            let pos_with_offset = math::Pos2::new(enemy.x + offset.x, enemy.y + offset.y);

            if player.x == pos_with_offset.x && player.y == pos_with_offset.y {
                assert!(false, "Enemy caught the player");
                math::Pos2::new(0, 0)
            } else {
                offset
            }
        }
    } else if player.x > enemy.x {
        let offset = math::Pos2::new(1, 0);
        let pos_with_offset = math::Pos2::new(enemy.x + offset.x, enemy.y + offset.y);
        if player.x == pos_with_offset.x && player.y == pos_with_offset.y {
            assert!(false, "Enemy caught the player");
            math::Pos2::new(0, 0)
        } else {
            offset
        }
    } else {
        let offset = math::Pos2::new(-1, 0);
        let pos_with_offset = math::Pos2::new(enemy.x + offset.x, enemy.y + offset.y);
        if player.x == pos_with_offset.x && player.y == pos_with_offset.y {
            assert!(false, "Enemy caught the player");
            math::Pos2::new(0, 0)
        } else {
            offset
        }
    }
}

impl GameState {
    fn new() -> Self {
        let mut repr_no_newline = [[GRASS_SYMBOL; CHUNK_WIDTH_NEWLINE]; CHUNK_HEIGHT];
        for i in 0..CHUNK_HEIGHT {
            repr_no_newline[i][CHUNK_WIDTH] = '\n';
        }
        Self {
            map_repr: repr_no_newline,
            player_pos: math::Pos2::new(0, 0),
            enemies: Vec::new(),
            enemy_index: 0,
        }
    }

    fn add_enemy(&mut self, pos: math::Pos2) -> usize {
        self.enemies.push(pos);
        let index = self.enemy_index;
        self.enemy_index += 1;
        index
    }

    //(NOTE): delete index and see if it will still work (it should)
    fn update_enemies(&mut self) {
        for (index, enemy) in self.enemies.iter_mut().enumerate() {
            let pos = find_closest_position_to_player(enemy, &self.player_pos);
            *enemy = *enemy + pos;
        }
    }

    fn render_enemies(&mut self) {
        for enemy in self.enemies.iter() {
            self.map_repr[enemy.y as usize][enemy.x as usize] = ENEMY_SYMBOL;
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
        self.map_repr[self.player_pos.y as usize][self.player_pos.x as usize] = PLAYER_SYMBOL;
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

            println!(
                "player pos ({}, {}) \n enemy pos ({}, {})",
                self.player_pos.x, self.player_pos.y, self.enemies[0].x, self.enemies[0].y
            );
        }
        true
    }

    fn flush_map(&mut self) {
        for i in 0..CHUNK_WIDTH_NEWLINE {
            for j in 0..CHUNK_HEIGHT {
                self.map_repr[j][i] = GRASS_SYMBOL;
            }
        }

        for i in 0..CHUNK_HEIGHT {
            self.map_repr[i][CHUNK_WIDTH] = '\n';
        }
    }

    fn render_map(&mut self) {
        self.flush_map();
        self.render_hero();
        self.render_enemies();

        let map_repr_1d: ArrayVec<[char; 1024]> = self.map_repr.iter().flatten().cloned().collect();
        let map_repr_string: String = map_repr_1d.iter().collect();
        println!("{}", map_repr_string);
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
