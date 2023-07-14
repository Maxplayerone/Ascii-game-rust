mod data_structures;
mod enemy;
mod inventory;
mod level_parser;
mod map;
mod math;
mod parser;
mod player;
mod weapons;
mod chest;

struct GameState {
    location_type: LocationType,

    map_manager: map::MapManager,
    inventory_manager: inventory::InventoryManager,
    player_manager: player::PlayerManager,
}

#[derive(PartialEq, Eq)]
pub enum LocationType {
    Map,
    Inventory,
}

impl GameState {
    fn new() -> Self {
        let (map_manager, player_pos) = map::MapManager::new();
        let inventory_manager = inventory::InventoryManager::new(inventory::DisableNodeRendering(false));
        let player_manager = player::PlayerManager::new(player_pos);
        Self {
            map_manager,
            inventory_manager,
            location_type: LocationType::Map,
            player_manager,
        }
    }

    fn update_location(&mut self) -> bool {
        match self.location_type {
            LocationType::Map => self
                .map_manager
                .update(&mut self.location_type, &mut self.player_manager),
            LocationType::Inventory => self.inventory_manager.update(&mut self.location_type),
        }
    }

    fn render_location(&mut self) {
        match self.location_type {
            LocationType::Map => self.map_manager.render(&self.player_manager),
            LocationType::Inventory => self.inventory_manager.render(&mut self.player_manager),
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
