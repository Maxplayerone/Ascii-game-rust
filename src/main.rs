mod data_structures;
mod enemy;
mod input_parser;
mod inventory;
mod level_parser;
mod map;
mod math;
mod player;

struct GameState {
    location_type: LocationType,

    map_manager: map::MapManager,
    player_manager: player::PlayerManager,
    enemy_manager: enemy::EnemyManager,
    inventory_manager: inventory::InventoryManager,
}

#[derive(PartialEq, Eq)]
pub enum LocationType {
    Map,
    Inventory,
}

pub enum ItemType {
    Rifle,
    SmallMed,
    BigMed,
    Sword,
    Shotgun,
}

impl ItemType {
    pub fn string(&self) -> &str {
        match self {
            ItemType::Rifle => "Rifle",
            ItemType::SmallMed => "Small med",
            ItemType::BigMed => "Big med",
            ItemType::Sword => "Sword",
            ItemType::Shotgun => "Shotgun",
        }
    }
}

const ENEMY_SYMBOL: char = '@';
const PLAYER_SYMBOL: char = '0';
const GRASS_SYMBOL: char = 'x';

impl GameState {
    fn new() -> Self {
        let (map_manager, info) = map::MapManager::new(PLAYER_SYMBOL, ENEMY_SYMBOL, GRASS_SYMBOL);
        let player_manager = player::PlayerManager::new(info.player);
        let enemy_manager = enemy::EnemyManager::new(info.enemies);
        let inventory_manager = inventory::InventoryManager::new();
        Self {
            map_manager,
            player_manager,
            enemy_manager,
            inventory_manager,
            location_type: LocationType::Map,
        }
    }

    fn update_location(&mut self) -> bool {
        match self.location_type {
            LocationType::Map => self.map_manager.update(
                &mut self.inventory_manager,
                &mut self.location_type,
                &mut self.enemy_manager,
                &mut self.player_manager,
            ),
            LocationType::Inventory => self.inventory_manager.update(&mut self.location_type),
        }
    }

    fn render_location(&mut self) {
        match self.location_type {
            LocationType::Map => self
                .map_manager
                .render(&mut self.player_manager, &mut self.enemy_manager),
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
