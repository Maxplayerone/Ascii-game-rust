use crate::{data_structures, parser, player, weapons, LocationType};

//(NOTE): every enemy has the same amount of health and damage
pub struct FightManager {
    enemy_health: i32,
    enemy_damage: usize,
    parser: parser::ParserManager<FightCommand>,
}

#[derive(Copy, Clone)]
enum FightCommand {
    Select(usize),
    Attack,
}

impl FightManager {
    pub fn new() -> Self {
        let attack = parser::WordProgress::new("attack".to_string(), FightCommand::Attack);
        let mut searched_words = Vec::new();
        searched_words.push(attack);
        for i in 0..10 {
            searched_words.push(parser::WordProgress::new(
                i.to_string(),
                FightCommand::Select(i),
            ));
        }
        let parser = parser::ParserManager::<FightCommand>::new(searched_words);
        Self {
            enemy_health: 50,
            enemy_damage: 20,
            parser,
        }
    }

    pub fn update(
        &mut self,
        player: &mut player::PlayerManager,
        location_type: &mut LocationType,
    ) -> bool {
        let queue: Option<data_structures::Queue<FightCommand>> = self.parser.parse();
        match queue {
            Some(mut queue) => {
                let command = queue.pop();
                match command {
                    FightCommand::Attack => {
                        if let Some(item_index) = player.current_selected_item {
                            let mut item = player.items[item_index];
                            self.enemy_health -= item.get_damage() as i32;
                            if item.decrease_durability(){
                                player.current_selected_item = None;
                            }
                            
                            if self.enemy_health <= 0 {
                                *location_type = LocationType::Map;
                                println!("Enemy dies");
                                return true;
                            }
                        } else {
                            println!("No item selected. Oops");
                        }
                    }
                    FightCommand::Select(num) => println!("Selecting {}", num),
                }
            }
            None => (),
        }
        true
    }

    pub fn render(&self) {
        println!("  ----");
        println!(r" / ||  \");
        println!(r"/  --   \");
        println!("----------");
        println!("Enemy slime attacks you!");
    }
}
