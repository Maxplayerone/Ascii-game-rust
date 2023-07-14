pub struct ItemDescriptor {
    pub durability: i32,
    pub damage: Option<i32>,
    pub healing: Option<i32>,
}

#[derive(Copy, Clone)]
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

    pub fn get_desc(&self) -> ItemDescriptor {
        match self {
            ItemType::Rifle => ItemDescriptor {
                durability: 5,
                damage: Some(30),
                healing: None,
            },
            ItemType::SmallMed => ItemDescriptor {
                durability: 1,
                damage: None,
                healing: Some(20),
            },
            ItemType::BigMed => ItemDescriptor {
                durability: 1,
                damage: None,
                healing: Some(50),
            },
            ItemType::Sword => ItemDescriptor {
                durability: 10,
                damage: Some(15),
                healing: None,
            },
            ItemType::Shotgun => ItemDescriptor {
                durability: 2,
                damage: Some(50),
                healing: None,
            },
        }
    }
}
