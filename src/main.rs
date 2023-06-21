use arrayvec::ArrayVec;
use std::ops::Add;


#[derive(Debug, Copy, Clone)]
struct Pos2{
    x: i16,
    y: i16,
}

impl Pos2{
    fn new(x: i16, y: i16) -> Self{
        Self{
            x,
            y,
        }
    }
}

impl Add for Pos2 {
    type Output = Pos2;
    fn add(self, rhs: Pos2) -> Pos2 {
        Pos2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
//(NOTE): how coordinate system works
//(0, 0) top-left
const CHUNK_WIDTH: usize = 32;
//we have to add one more for the newline
const CHUNK_WIDTH_NEWLINE: usize = CHUNK_WIDTH + 1;
const CHUNK_HEIGHT: usize = 30;

struct GameState {
    map_repr: [[char; CHUNK_WIDTH_NEWLINE]; CHUNK_HEIGHT],
    player_pos: Pos2,
    enemies: Vec<Pos2>,
    enemy_index: usize,
}

const ENEMY_SYMBOL: char = '@';

enum InputCommand{
    PlayerRight,
    PlayerLeft,
    PlayerUp,
    PlayerDown,
}

impl GameState {
    fn new() -> Self {
        let mut repr_no_newline = [['x'; CHUNK_WIDTH_NEWLINE]; CHUNK_HEIGHT];
        for i in 0..CHUNK_HEIGHT {
            repr_no_newline[i][CHUNK_WIDTH] = '\n';
        }
        Self {
            map_repr: repr_no_newline,
            player_pos: Pos2::new(0, 0),
            enemies: Vec::new(),
            enemy_index: 0,
        }
    }

    fn add_enemy(&mut self, pos: Pos2) -> usize{
        self.enemies.push(pos);
        let index = self.enemy_index;
        self.enemy_index += 1; 
        index
    }

    fn update_enemy(&mut self, enemy_index: usize, pos: Pos2){
        self.enemies[enemy_index] = self.enemies[enemy_index] + pos;
    }

    fn render_enemies(&mut self){
        for enemy in self.enemies.iter(){
            self.map_repr[enemy.y as usize][enemy.x as usize] = ENEMY_SYMBOL;
        }
    }

    fn update_hero(&mut self, input_command: InputCommand){
        match input_command{
            InputCommand::PlayerRight => self.player_pos.x += 1,
            InputCommand::PlayerLeft => self.player_pos.x -= 1,
            InputCommand::PlayerUp => self.player_pos.y -= 1,
            InputCommand::PlayerDown => self.player_pos.y += 1,
        }
    }

    fn render_hero(&mut self) {
        self.map_repr[self.player_pos.y as usize][self.player_pos.x as usize] = '0';
    }

    fn update_map(&mut self, command: String) -> bool{
        if command.contains("right"){
            self.update_hero(InputCommand::PlayerRight);
            return true;
        }else if command.contains("left"){
            self.update_hero(InputCommand::PlayerLeft);
            return true;
        }else if command.contains("up"){
            self.update_hero(InputCommand::PlayerUp);
            return true;
        }else if command.contains("down"){
            self.update_hero(InputCommand::PlayerDown);
            return true;
        }else if command.contains("quit"){
            return false;
        }else if command.contains("wait"){
            return true;
        }

        true
    }

    fn flush_map(&mut self){
        for i in 0..CHUNK_WIDTH_NEWLINE{
            for j in 0..CHUNK_HEIGHT{
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

        //(NOTE): rendering the rest of the map
        //how flatten and cloned works
        //flatten- changes a multiply nested iterator into one iterator
        //cloned- we create a new array so next the collect method has char and not &char elements (I'm not sure about that one)
        let map_repr_1d: ArrayVec<[char; 1024]> = self.map_repr.iter().flatten().cloned().collect();
        //how collect works here
        //in this example we are changing ArrayVec or a vector of chars into a string
        //collect is used to change use collection into another
        let map_repr_string: String = map_repr_1d.iter().collect();
        println!("{}", map_repr_string);
    }
}

fn read_user_input() -> String{
    let mut line = String::new();
    //(NOTE) byte_size: number of characters read + 2 (one is for entry and the other idk)
    let _byte_size = std::io::stdin().read_line(&mut line).unwrap();
    line
}

fn main() {
    let mut state = GameState::new();
    let enemy_index = state.add_enemy(Pos2::new(10, 24));
    let mut playing = true;
    while playing == true{
        state.update_enemy(enemy_index, Pos2::new(0, -1));
        state.render_map();
        playing = state.update_map(read_user_input());
    }
}
