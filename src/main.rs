mod chest;
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
    player_manager: player::PlayerManager,
}

#[derive(PartialEq, Eq)]
pub enum LocationType {
    Map,
    Inventory,
}

impl GameState {
    fn new(name: String) -> Self {
        let (map_manager, player_pos) = map::MapManager::new();
        let inventory_manager =
            inventory::InventoryManager::new(inventory::DisableNodeRendering(false), inventory::DisablePlayerStatsRendering(true));
        let mut player_manager = player::PlayerManager::new(player_pos, name, 100);

        player_manager.add_item(weapons::ItemType::Rifle);
        player_manager.add_item(weapons::ItemType::BigMed);
        player_manager.add_item(weapons::ItemType::Shotgun);

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
            LocationType::Inventory => self.inventory_manager.update(&mut self.location_type, &mut self.player_manager),
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
    println!("What's your name: ");
    let mut name = String::new();
    let _byte_size = std::io::stdin().read_line(&mut name).unwrap();

    let mut state = GameState::new(name);
    let mut playing = true;
    while playing {
        state.render_location();
        playing = state.update_location();
    }
}
