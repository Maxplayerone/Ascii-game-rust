use crate::{data_structures, parser, player, LocationType, weapons};

//(NOTE): every enemy has the same amount of health and damage
pub struct FightManager {
    enemy_health: i32,
    enemy_damage: i32,
    parser: parser::ParserManager<FightCommand>,
}

#[derive(Copy, Clone)]
enum FightCommand {
    Select(usize),
    Attack,
    Inv,
    Quit,
}

fn render_item(item: &weapons::Item){
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

impl FightManager {
    pub fn new() -> Self {
        let attack = parser::WordProgress::new("attack".to_string(), FightCommand::Attack);
        let inv = parser::WordProgress::new("inv".to_string(), FightCommand::Inv);
        let quit = parser::WordProgress::new("quit".to_string(), FightCommand::Quit);
        
        let mut searched_words = Vec::new();
        searched_words.push(attack);
        searched_words.push(inv);
        searched_words.push(quit);

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
                    FightCommand::Select(num) => {
                        if num >= player.items.len(){
                            println!("You've picked a bigger number than expected!");
                        }else{
                            player.current_selected_item = Some(num);
                        }
                    },
                    FightCommand::Inv => {
                        for item in &player.items{
                            render_item(item);
                        }
                        return self.update(player, location_type);
                    }
                    FightCommand::Quit => return false,
                }
            }
            None => (),
        }
        player.health -= self.enemy_damage;
        if player.health <= 0{
            println!("You died!");
            return false;
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
