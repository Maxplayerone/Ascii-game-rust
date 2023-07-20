use crate::data_structures;
use crate::parser;
use crate::player;
use crate::weapons;
use crate::LocationType;

fn render_player_stats(player: &player::PlayerManager) {
    println!("-------------------------");
    println!("           *      *");
    println!("           --------");
    println!("Name: {}", player.get_name());
    println!("Health: {}", player.health);
    println!("Item count: {}", player.get_item_count());
    println!("-------------------------\n");
}

fn render_item_node(item: &weapons::Item) {
    let descriptor = item.get_desc();

    println!("------------------------------");
    println!("Item: {}", item.string());
    println!("Durability: {}", descriptor.durability);

    match descriptor.damage {
        Some(damage) => println!("Damage: {}", damage),
        None => println!("Damage: None"),
    }

    match descriptor.healing {
        Some(healing) => println!("Healing: {}", healing),
        None => println!("Healing: None"),
    }

    println!("------------------------------");
}

#[derive(PartialEq, Copy, Clone)]
enum InvCommand {
    Map,
    Quit,
    Equip,
    Drop,
    Select(usize),
    Tut,
}

pub struct InventoryManager {
    node_rendering_flag: DisableNodeRendering,
    player_stats_flag: DisablePlayerStatsRendering,
    parser: parser::ParserManager<InvCommand>,
    last_number_selected: Option<usize>,
}

#[derive(PartialEq)]
pub struct DisableNodeRendering(pub bool);

#[derive(PartialEq)]
pub struct DisablePlayerStatsRendering(pub bool);

impl InventoryManager {
    pub fn new(
        node_rendering_flag: DisableNodeRendering,
        player_stats_flag: DisablePlayerStatsRendering,
    ) -> Self {
        let parser = Self::setup_basic_inventory_parser();
        Self {
            node_rendering_flag,
            player_stats_flag,
            parser,
            last_number_selected: None,
        }
    }

    fn setup_basic_inventory_parser() -> parser::ParserManager<InvCommand> {
        let map_progress = parser::WordProgress::new("map".to_string(), InvCommand::Map);
        let quit_progress = parser::WordProgress::new("quit".to_string(), InvCommand::Quit);
        let equip_progress = parser::WordProgress::new("equip".to_string(), InvCommand::Equip);
        let drop_progress = parser::WordProgress::new("drop".to_string(), InvCommand::Drop);
        let tutorial_progress = parser::WordProgress::new("tut".to_string(), InvCommand::Tut);

        let mut searched_words = Vec::new();
        searched_words.push(map_progress);
        searched_words.push(quit_progress);
        searched_words.push(equip_progress);
        searched_words.push(drop_progress);
        searched_words.push(tutorial_progress);
        parser::ParserManager::<InvCommand>::new(searched_words)
    }

    pub fn render(&mut self, player: &mut player::PlayerManager) {
        //we have player here cuz the game renders first and then updates
        if self.node_rendering_flag == DisableNodeRendering(false) {
            for item in &player.items {
                render_item_node(item);
            }
        }

        if self.player_stats_flag == DisablePlayerStatsRendering(false) {
            render_player_stats(&player);
        }
    }

    pub fn update(
        &mut self,
        location_type: &mut LocationType,
        player: &mut player::PlayerManager,
    ) -> bool {
        //adding new WordProgress items
        if player.got_new_item {
            self.parser = Self::setup_basic_inventory_parser();

            for i in 0..player.item_count() {
                self.parser.add_word(parser::WordProgress::new(
                    i.to_string(),
                    InvCommand::Select(i),
                ));
            }

            player.set_got_new_item(false);
        }

        let queue: Option<data_structures::Queue<InvCommand>> = self.parser.parse();

        match queue {
            Some(mut queue) => {
                for _ in 0..queue.size() {
                    let command = queue.pop();
                    match command {
                        InvCommand::Map => {
                            *location_type = LocationType::Map;
                            println!("Going to map");
                        }
                        InvCommand::Quit => return false,
                        InvCommand::Drop => {
                            if let Some(index) = self.last_number_selected {
                                if player.current_selected_item == Some(index) {
                                    player.current_selected_item = None;
                                }
                                player.remove_item(index);
                            } else {
                                println!("Please select an item slot");
                            }
                        }
                        InvCommand::Equip => {
                            if let Some(index) = self.last_number_selected {
                                player.current_selected_item = Some(index);
                            } else {
                                println!("Please select an item slot");
                            }
                        }
                        InvCommand::Select(item_number) => {
                            self.last_number_selected = Some(item_number);
                        }
                        InvCommand::Tut => {
                            println!("----------------------------------------------------");
                            println!("              Inventory State tutorial");
                            println!("drop - dropping selected item");
                            println!("equip - equipping selected item");
                            println!("To select an item you type the number of the item\n (the item at the bottom is 0, then it's 1 etc)");
                            println!("tut - showing this tutorial :>");
                            println!("map - changing state to map");
                            println!("quit - quitting the game\n");
                            println!("----------------------------------------------------");
                        }
                    }
                }
            }
            None => (),
        }
        true
    }
}
