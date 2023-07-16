use crate::data_structures;
use crate::parser;
use crate::player;
use crate::weapons;
use crate::LocationType;

#[derive(Copy, Clone)]
struct Node {
    item_type: weapons::ItemType,
}

impl Node {
    fn new(item_type: weapons::ItemType) -> Self {
        Self { item_type }
    }

    fn render(&self) {
        let descriptor = self.item_type.get_desc();

        println!("------------------------------");
        println!("Item: {}", self.item_type.string());
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
}

fn render_player_stats(player: &player::PlayerManager) {
    println!("-------------------------");
    println!("           *      *");
    println!("           --------");
    println!("Name: {}", player.get_name());
    println!("Health: {}", player.get_health());
    println!("Item count: {}", player.get_item_count());
    println!("-------------------------\n");
}

#[derive(PartialEq, Copy, Clone)]
enum InvCommand {
    Map,
    Quit,
    Equip,
    Drop,
    Select(usize),
    Tutorial,
}

pub struct InventoryManager {
    nodes: Vec<Node>,
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
        let map_progress = parser::WordProgress::new("map".to_string(), InvCommand::Map);
        let quit_progress = parser::WordProgress::new("quit".to_string(), InvCommand::Quit);
        let equip_progress = parser::WordProgress::new("equip".to_string(), InvCommand::Equip);
        let drop_progress = parser::WordProgress::new("drop".to_string(), InvCommand::Drop);
        let tutorial_progress = parser::WordProgress::new("info".to_string(), InvCommand::Tutorial);

        let mut searched_words = Vec::new();
        searched_words.push(map_progress);
        searched_words.push(quit_progress);
        searched_words.push(equip_progress);
        searched_words.push(drop_progress);
        searched_words.push(tutorial_progress);
        let parser = parser::ParserManager::<InvCommand>::new(searched_words);

        Self {
            node_rendering_flag,
            player_stats_flag,
            nodes: Vec::new(),
            parser,
            last_number_selected: None,
        }
    }

    pub fn add_node(&mut self, item_type: weapons::ItemType) {
        self.nodes.push(Node::new(item_type));

        let length = self.nodes.len() - 1;

        self.parser.add_word(parser::WordProgress::new(
            length.to_string(),
            InvCommand::Select(length),
        ));
    }

    pub fn render(&mut self, player: &mut player::PlayerManager) {
        if player.get_new_item_bool() {
            self.add_node(player.get_most_recent_item());
        }

        if self.node_rendering_flag == DisableNodeRendering(false) {
            for node in self.nodes.iter() {
                node.render();
            }
        }

        if self.player_stats_flag == DisablePlayerStatsRendering(false) {
            render_player_stats(&player);
        }
    }

    pub fn update(&mut self, location_type: &mut LocationType) -> bool {
        //adding new items

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
                                let size = self.nodes.len();
                                if index >= size{
                                    println!("Index out of bounds");
                                }else{
                                    self.nodes[index] = self.nodes[size - 1];
                                    self.nodes.pop();
                                }
                            }else{
                                println!("Please select an item slot");
                            }
                        }
                        InvCommand::Equip => println!("Equppin"),
                        InvCommand::Select(item_number) => {
                            self.last_number_selected = Some(item_number);
                        }
                        InvCommand::Tutorial => {
                            println!("Tutorial");
                        }
                    }
                }
            }
            None => (),
        }
        true
    }
}
