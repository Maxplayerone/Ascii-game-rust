use crate::{parser, data_structures};

//(NOTE): every enemy has the same amount of health and damage
pub struct FightManager{
    enemy_health: usize,
    enemy_damage: usize,
    parser: parser::ParserManager<FightCommand>,
}

#[derive(Copy, Clone)]
enum FightCommand{
    Select(usize),
    Attack,
}

impl FightManager{
    pub fn new() -> Self{
        let attack = parser::WordProgress::new("attack".to_string(), FightCommand::Attack);
        let mut searched_words = Vec::new();
        searched_words.push(attack);
        for i in 0..10{
            searched_words.push(parser::WordProgress::new(i.to_string(), FightCommand::Select(i)));
        }
        let parser = parser::ParserManager::<FightCommand>::new(searched_words);
        Self{
            enemy_health: 50,            
            enemy_damage: 20,
            parser,
        }
    }
    
    pub fn update(&mut self) -> bool{
        let queue: Option<data_structures::Queue<FightCommand>> = self.parser.parse();
        match queue{
            Some(mut queue) => {
                let command = queue.pop();
                match command{
                    FightCommand::Attack => println!("Attacking"),
                    FightCommand::Select(num) => println!("Selecting {}", num),
                }
            }
            None => (),
        }        
        true
    }
    
    pub fn render(&self){
        println!("  ----");
        println!(r" / ||  \");
        println!(r"/  --   \");
        println!("----------");
        println!("Enemy slime attacks you!");
    }
}
