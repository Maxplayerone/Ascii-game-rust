mod data_structures;
mod enemy;
mod input_parser;
mod level_parser;
mod math;
mod inventory;

//(NOTE): how coordinate system works
//(0, 0) top-left

struct GameState {
    location_type: LocationType,
    map: Vec<char>,
    map_dimensions: math::Pos2,
    player_pos: math::Pos2,

    enemy_manager: enemy::EnemyManager,
    inventory_manager: inventory::InventoryManager,
}

#[derive(PartialEq, Eq)]
pub enum LocationType {
    Map,
    Inventory,
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
        let mut inventory_manager = inventory::InventoryManager::new();
        inventory_manager.add_node();
        Self {
            map,
            player_pos,
            map_dimensions,
            enemy_manager,
            location_type: LocationType::Map,
            inventory_manager,
        }
    }

    fn update_enemies(&mut self) {
        self.enemy_manager.update_enemies(&self.player_pos);
    }

    fn render_enemies(&mut self) {
        let width_usize: usize = self.map_dimensions.x.try_into().unwrap();

        for i in 0..self.enemy_manager.size() {
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

    fn update_location(&mut self) -> bool {
        match self.location_type {
            LocationType::Map => self.update_map(),
            LocationType::Inventory => self.inventory_manager.update(),
        }
    }

    fn update_map(&mut self) -> bool {
        let (mut queue, location_changer) = input_parser::get_parsed_user_input_map();
        if let Some(location_changer) = location_changer{
            self.location_type = location_changer;
        }

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

    fn render_location(&mut self) {
        match self.location_type {
            LocationType::Map => self.render_map(),
            LocationType::Inventory => self.inventory_manager.render(),
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
        state.render_location();
        playing = state.update_location();
    }
}
