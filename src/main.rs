use arrayvec::ArrayVec;

//(NOTE): how coordinate system works
//(0, 0) top-left
//(NOTE) usize- how many bytes a pointer on a given architecture is
//64 bit machine- 8 bytes, 32 bit target- 4 bytes
const CHUNK_WIDTH: usize = 8;
//we have to add one more for the newline
const CHUNK_WIDTH_NEWLINE: usize = 9;
const CHUNK_HEIGHT: usize = 8;

struct Pos2{
    x: usize,
    y: usize,
}

impl Pos2{
    fn new(x: usize, y: usize) -> Self{
        Self{
            x,
            y,
        }
    }
}

struct GameState {
    map_repr: [[char; CHUNK_WIDTH_NEWLINE]; CHUNK_HEIGHT],
    player_pos: Pos2,
}

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
            repr_no_newline[i][8] = '\n';
        }
        Self {
            map_repr: repr_no_newline,
            player_pos: Pos2::new(0, 0),
        }
    }

    fn flush_map(&mut self){
        for i in 0..CHUNK_WIDTH_NEWLINE{
            for j in 0..CHUNK_HEIGHT{
                self.map_repr[j][i] = 'x';
           }
        }

        for i in 0..CHUNK_HEIGHT {
            self.map_repr[i][8] = '\n';
        }
    }

    fn render_map(&mut self) {
        self.flush_map();
        self.render_hero();

        //(NOTE): rendering the rest of the map
        //how flatten and cloned works
        //flatten- changes a multiply nested iterator into one iterator
        //cloned- we create a new array so next the collect method has char and not &char elements (I'm not sure about that one)
        let map_repr_1d: ArrayVec<[char; 72]> = self.map_repr.iter().flatten().cloned().collect();
        //how collect works here
        //in this example we are changing ArrayVec or a vector of chars into a string
        //collect is used to change use collection into another
        let map_repr_string: String = map_repr_1d.iter().collect();
        println!("{}", map_repr_string);
    }

    fn render_hero(&mut self) {
        self.map_repr[self.player_pos.y][self.player_pos.x] = '0';
    }

    fn update_hero(&mut self, input_command: InputCommand){
        match input_command{
            InputCommand::PlayerRight => self.player_pos.x += 1,
            InputCommand::PlayerLeft => self.player_pos.x -= 1,
            InputCommand::PlayerUp => self.player_pos.y -= 1,
            InputCommand::PlayerDown => self.player_pos.y += 1,
        }
    }

    fn update_scene(&mut self, command: String) -> bool{
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
        }

        true
    }
}

fn read_user_input() -> String{
    let mut line = String::new();
    //(NOTE) byte_size: number of characters read + 2 (one is for entry and the other idk)
    let byte_size = std::io::stdin().read_line(&mut line).unwrap();
    line
}

fn main() {
    let mut state = GameState::new();
    let mut playing = true;
    while playing == true{
        state.render_map();
        playing = state.update_scene(read_user_input());
    }
}
