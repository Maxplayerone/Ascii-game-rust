mod data_structures;
mod enemy;
mod inventory;
mod level_parser;
mod map;
mod math;
mod parser;
mod player;
mod weapons;

struct GameState {
    location_type: LocationType,

    map_manager: map::MapManager,
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
        let map_manager = map::MapManager::new(PLAYER_SYMBOL, ENEMY_SYMBOL, GRASS_SYMBOL);
        let inventory_manager = inventory::InventoryManager::new(true);
        Self {
            map_manager,
            inventory_manager,
            location_type: LocationType::Map,
        }
    }

    fn update_location(&mut self) -> bool {
        match self.location_type {
            LocationType::Map => self.map_manager.update(&mut self.location_type),
            LocationType::Inventory => self.inventory_manager.update(&mut self.location_type),
        }
    }

    fn render_location(&mut self) {
        match self.location_type {
            LocationType::Map => self.map_manager.render(),
            LocationType::Inventory => self.inventory_manager.render(),
        }
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
