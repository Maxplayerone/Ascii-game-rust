mod chest;
mod data_structures;
mod enemy;
mod fight;
mod inventory;
mod level_parser;
mod map;
mod math;
mod menu;
mod parser;
mod player;
mod weapons;

struct GameState {
    location_type: LocationType,

    map_manager: map::MapManager,
    inventory_manager: inventory::InventoryManager,
    fight_manager: fight::FightManager,

    player_manager: player::PlayerManager,
}

#[derive(PartialEq, Eq)]
pub enum LocationType {
    Map,
    Inventory,
    Fight,
}

impl GameState {
    fn new(name: String, filename: String) -> Self {
        let (map_manager, player_pos) = map::MapManager::new(filename);
        let inventory_manager = inventory::InventoryManager::new(
            inventory::DisableNodeRendering(false),
            inventory::DisablePlayerStatsRendering(false),
        );
        let fight_manager = fight::FightManager::new();
        let player_manager = player::PlayerManager::new(player_pos, name, 100);

        Self {
            map_manager,
            inventory_manager,
            location_type: LocationType::Map,
            player_manager,
            fight_manager,
        }
    }

    fn update_location(&mut self) -> bool {
        match self.location_type {
            LocationType::Map => self
                .map_manager
                .update(&mut self.location_type, &mut self.player_manager),
            LocationType::Inventory => self
                .inventory_manager
                .update(&mut self.location_type, &mut self.player_manager),
            LocationType::Fight => self
                .fight_manager
                .update(&mut self.player_manager, &mut self.location_type),
        }
    }

    fn render_location(&mut self) {
        match self.location_type {
            LocationType::Map => self.map_manager.render(&self.player_manager),
            LocationType::Inventory => self.inventory_manager.render(&mut self.player_manager),
            LocationType::Fight => self.fight_manager.render(),
        }
    }
}

fn main() {
    println!("What's your name: ");
    let mut name = String::new();
    let _byte_size = std::io::stdin().read_line(&mut name).unwrap();

    let mut prestate = menu::MenuState::new();
    let mut filename: Option<String> = None;
    while filename.is_none() {
        prestate.render();
        filename = prestate.update();
    }

    if Some("quit".to_string()) != filename {
        if let Some(filename) = filename {
            let mut state = GameState::new(name, filename);
            let mut playing = true;
            while playing {
                state.render_location();
                playing = state.update_location();
            }
        }
    }
}
