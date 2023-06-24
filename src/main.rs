use arrayvec::ArrayVec;

mod data_structures;
mod math;
mod input_parser;

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

//(NOTE): the algorith works by moving enemy to player's y axis and then moving verticallly
//I think I would prefer to move enemy by the axis that is closest to player
//aka if the difference in (2, 4) we should move vertically (changing y)
fn find_closest_position_to_player(enemy: &math::Pos2, player: &math::Pos2) -> math::Pos2 {
    let diff_x = (enemy.x - player.x).abs();
    let diff_y = (enemy.y - player.y).abs();
    println!("Diff_x {} diff_y {}", diff_x, diff_y);

    //we are standing on the player
    if diff_x == 0 && diff_y == 0 {
        println!("We are standing on the player");
        return math::Pos2::new(0, 0);
    }

    //we only need to move up and down now
    if diff_x == 0 {
        if enemy.y > player.y {
            math::Pos2::new(0, -1)
        } else {
            math::Pos2::new(0, 1)
        }
    } else if enemy.x > player.x {
        math::Pos2::new(-1, 0)
    } else {
        math::Pos2::new(1, 0)
    }
}

impl GameState {
    fn new() -> Self {
        let mut repr_no_newline = [['x'; CHUNK_WIDTH_NEWLINE]; CHUNK_HEIGHT];
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
                let x = 3;
            }
        }
    }

    fn render_hero(&mut self) {
        self.map_repr[self.player_pos.y as usize][self.player_pos.x as usize] = '0';
    }

    fn update_map(&mut self){
        let mut queue = input_parser::get_parsed_user_input_map();
        
            for _ in 0..queue.size() {
                let command: input_parser::InputCommand = queue.pop();
                if command == input_parser::InputCommand::Quit {
                    assert!(false);
                } else {
                    self.update_hero(command);
                }

                self.update_enemies();
            }
    }

    fn flush_map(&mut self) {
        for i in 0..CHUNK_WIDTH_NEWLINE {
            for j in 0..CHUNK_HEIGHT {
                self.map_repr[j][i] = 'x';
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
    //let enemy_index = state.add_enemy(math::Pos2::new(3, 3));
    loop{
        state.render_map();
        state.update_map();
    }
}
