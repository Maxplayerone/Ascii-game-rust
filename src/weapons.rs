pub struct ItemDescriptor {
    pub durability: usize,
    pub damage: Option<usize>,
    pub healing: Option<usize>,
}

#[derive(Copy, Clone)]
pub enum ItemType {
    Rifle,
    SmallMed,
    BigMed,
    Sword,
    Shotgun,
}

const RIFLE_DAMAGE: usize = 30;
const SWORD_DAMAGE: usize = 15;
const SHOTGUN_DAMAGE: usize = 50;

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

    pub fn get_damage(&self) -> usize{
        match self{
            ItemType::Rifle => RIFLE_DAMAGE,
            ItemType::Sword => SWORD_DAMAGE,
            ItemType::Shotgun => SHOTGUN_DAMAGE,
            _ => {
                println!("This weapon doesn't do any damage!");
                0
            }
        }
    }

    pub fn get_desc(&self) -> ItemDescriptor {
        match self {
            ItemType::Rifle => ItemDescriptor {
                durability: 5,
                damage: Some(RIFLE_DAMAGE),
                healing: None,
            },
            ItemType::SmallMed => ItemDescriptor {
                durability: 1,
                damage: None,
                healing: Some(25),
            },
            ItemType::BigMed => ItemDescriptor {
                durability: 1,
                damage: None,
                healing: Some(50),
            },
            ItemType::Sword => ItemDescriptor {
                durability: 10,
                damage: Some(SWORD_DAMAGE),
                healing: None,
            },
            ItemType::Shotgun => ItemDescriptor {
                durability: 2,
                damage: Some(SHOTGUN_DAMAGE),
                healing: None,
            },
        }
    }
}
