use arrayvec::ArrayVec;
use std::ops::Add;

#[derive(Debug, Copy, Clone)]
struct Pos2 {
    x: i16,
    y: i16,
}

impl Pos2 {
    fn new(x: i16, y: i16) -> Self {
        Self { x, y }
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

struct Queue{
    commands: Vec<InputCommand>,
    marker: usize, //points to the first element in the queue
}

impl Queue{
    fn new() -> Self{
        Self{
            commands: Vec::new(),
            marker: 0,
        }
    }

    fn push(&mut self, command: InputCommand){
        self.commands.push(command);
    }

    fn pop(&mut self) -> InputCommand{
        let old_marker = self.marker;
        self.marker += 1;
        self.commands[old_marker]
    }

    fn get_number_of(&self, command: InputCommand) -> usize{
        let mut total_num: usize = 0;
        for element in self.commands.iter(){
            if *element == command{
                total_num += 1;
            }
        }
        total_num
    }

    fn size(&self) -> usize{
        self.commands.len()
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum InputCommand {
    Right,
    Left,
    Up,
    Down,
    Wait,
    Quit
}

fn find_closest_position_to_player(enemy: &Pos2, player: &Pos2) -> Pos2 {
    let diff_x = (enemy.x - player.x).abs();
    let diff_y = (enemy.y - player.y).abs();
    println!("Diff_x {} diff_y {}", diff_x, diff_y);

    //we are standing on the player
    if diff_x == 0 && diff_y == 0 {
        println!("We are standing on the player");
        return Pos2::new(0, 0);
    }

    //we only need to move up and down now
    if diff_x == 0 {
        if enemy.y > player.y {
            return Pos2::new(0, -1);
        } else {
            return Pos2::new(0, 1);
        }
    } else {
        if enemy.x > player.x {
            return Pos2::new(-1, 0);
        } else {
            return Pos2::new(1, 0);
        }
    }
}

fn check_for_starting_command_letters(c: &char) -> bool {
    c == &'r' || c == &'w' || c == &'d' || c == &'u' || c == &'l' || c == &'q'
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

    fn add_enemy(&mut self, pos: Pos2) -> usize {
        self.enemies.push(pos);
        let index = self.enemy_index;
        self.enemy_index += 1;
        index
    }

    fn update_enemies(&mut self) {
        for (index, enemy) in self.enemies.iter_mut().enumerate() {
            let pos = find_closest_position_to_player(&enemy, &self.player_pos);
            *enemy = *enemy + pos;
        }
    }

    fn render_enemies(&mut self) {
        for enemy in self.enemies.iter() {
            self.map_repr[enemy.y as usize][enemy.x as usize] = ENEMY_SYMBOL;
        }
    }

    fn update_hero(&mut self, input_command: InputCommand) {
        match input_command {
            InputCommand::Right => self.player_pos.x += 1,
            InputCommand::Left => self.player_pos.x -= 1,
            InputCommand::Up => self.player_pos.y -= 1,
            InputCommand::Down => self.player_pos.y += 1,
            _ => {let x = 3;},
        }
    }

    fn render_hero(&mut self) {
        self.map_repr[self.player_pos.y as usize][self.player_pos.x as usize] = '0';
    }

    fn update_map(&mut self, command: String) -> bool {
        //(NOTE) commands available in map mode
        let mut current_number = 1;
        let mut queue = Queue::new();


        let command: String = command.chars().filter(|c| !c.is_whitespace()).collect();
        for mut i in 0..command.chars().count() {
            let c = command.chars().nth(i).unwrap();

            //(NOTE): doesn't work for two and more digit numbers
            //(NOTE): there are some problems with the enemies
            if c.is_numeric() {
                current_number = c as u32 - '0' as u32;
            } else if check_for_starting_command_letters(&c) {
                //right
                if c == 'r' && command.chars().nth(i + 1).unwrap() == 'i' &&
                    command.chars().nth(i + 2).unwrap() == 'g' && 
                    command.chars().nth(i + 3).unwrap() == 'h' &&
                    command.chars().nth(i + 4).unwrap() == 't'
                {
                    for _ in 0..current_number{
                        queue.push(InputCommand::Right);
                    }
                    current_number = 1;
                    i += 4;
                    continue;
                }
                //left
                else if c == 'l' &&
                        command.chars().nth(i + 1).unwrap() == 'e' &&
                        command.chars().nth(i + 2).unwrap() == 'f' &&
                        command.chars().nth(i + 3).unwrap() == 't'
                {
                    for _ in 0..current_number{
                        queue.push(InputCommand::Left);
                    }
                    current_number = 1;
                    i += 3;
                    continue;
                }
                //up
                else if c == 'u' &&
                    command.chars().nth(i + 1).unwrap() == 'p'
                {
                    for _ in 0..current_number{
                        queue.push(InputCommand::Up);
                    }
                    current_number = 1;
                    i += 1;
                    continue;
                }
                //down
                else if c == 'd' &&
                    command.chars().nth(i + 1).unwrap() == 'o' &&
                    command.chars().nth(i + 2).unwrap() == 'w' &&
                    command.chars().nth(i + 3).unwrap() == 'n'
                {
                    for _ in 0..current_number{
                        queue.push(InputCommand::Down);
                    }
                    current_number = 1;
                    i += 3;
                    continue;

                }
                //wait
                else if c == 'w' &&
                    command.chars().nth(i + 1).unwrap() == 'a' &&
                    command.chars().nth(i + 2).unwrap() == 'i' &&
                    command.chars().nth(i + 3).unwrap() == 't'
                {
                    for _ in 0..current_number{
                        queue.push(InputCommand::Wait);
                    }
                    current_number = 1;
                    i += 3;
                    continue;

                }
                //quit
                else if c == 'q' &&
                    command.chars().nth(i + 1).unwrap() == 'u' &&
                    command.chars().nth(i + 2).unwrap() == 'i' &&
                    command.chars().nth(i + 3).unwrap() == 't'
                {
                    queue.push(InputCommand::Quit);
                    current_number = 1;
                    println!("quit");
                    i += 3;
                    continue;

                }
            }
        }
        println!("right amount {}", queue.get_number_of(InputCommand::Right));
        
        for _ in 0..queue.size(){
            let command: InputCommand = queue.pop();
            if command == InputCommand::Quit{
                assert!(false);
            }else{
                self.update_hero(command);
            }

            self.update_enemies();
        }
        true
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

fn read_user_input() -> String {
    let mut line = String::new();
    //(NOTE) byte_size: number of characters read + 2 (one is for entry and the other idk)
    let _byte_size = std::io::stdin().read_line(&mut line).unwrap();
    line
}

fn main() {
    let mut state = GameState::new();
    //let enemy_index = state.add_enemy(Pos2::new(3, 3));
    let mut playing = true;
    while playing == true {
        state.render_map();
        playing = state.update_map(read_user_input());
    }
}
