pub struct ItemDescriptor {
    pub durability: i32,
    pub damage: Option<usize>,
    pub healing: Option<usize>,
}

#[derive(Copy, Clone, Debug)]
pub enum ItemType {
    Rifle,
    SmallMed,
    BigMed,
    Sword,
    Shotgun,
}

const RIFLE_DAMAGE: usize = 30;
const RIFLE_DURABILITY: i32 = 5;

const SWORD_DAMAGE: usize = 15;
const SWORD_DURABILITY: i32 = 10;

const SHOTGUN_DAMAGE: usize = 50;
const SHOTGUN_DURABILITY: i32 = 2;

const SMALL_MED_DURABILITY: i32 = 1;
const BIG_MED_DURABILITY: i32 = 1;

#[derive(Copy, Clone)]
pub struct Item {
    item_type: ItemType,
    current_durability: i32,
}

impl Item {
    pub fn new(item: ItemType) -> Self {
        Self {
            current_durability: item.get_durability(),
            item_type: item,
        }
    }

    pub fn decrease_durability(&mut self) -> bool {
        self.current_durability -= 1;
        if self.current_durability <= 0 {
            true
        } else {
            false
        }
    }

    pub fn get_damage(&self) -> usize {
        self.item_type.get_damage()
    }

    pub fn string(&self) -> &str {
        self.item_type.string()
    }

    pub fn get_desc(&self) -> ItemDescriptor {
        self.item_type.get_desc()
    }

    pub fn get_health(&self) -> i32{
        self.item_type.get_health()
    }
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

    pub fn get_damage(&self) -> usize {
        match self {
            ItemType::Rifle => RIFLE_DAMAGE,
            ItemType::Sword => SWORD_DAMAGE,
            ItemType::Shotgun => SHOTGUN_DAMAGE,
            _ => {
                println!("This weapon doesn't do any damage!");
                0
            }
        }
    }

    pub fn get_durability(&self) -> i32 {
        match self {
            ItemType::Rifle => RIFLE_DURABILITY,
            ItemType::Sword => SWORD_DURABILITY,
            ItemType::Shotgun => SHOTGUN_DURABILITY,
            ItemType::BigMed => SMALL_MED_DURABILITY,
            ItemType::SmallMed => BIG_MED_DURABILITY,
        }
    }

    pub fn get_health(&self) -> i32 {
        match self {
            ItemType::SmallMed => 25,
            ItemType::BigMed => 50,
             _ => 0,
        }
    }

    pub fn get_desc(&self) -> ItemDescriptor {
        match self {
            ItemType::Rifle => ItemDescriptor {
                durability: RIFLE_DURABILITY,
                damage: Some(RIFLE_DAMAGE),
                healing: None,
            },
            ItemType::SmallMed => ItemDescriptor {
                durability: SMALL_MED_DURABILITY,
                damage: None,
                healing: Some(25),
            },
            ItemType::BigMed => ItemDescriptor {
                durability: BIG_MED_DURABILITY,
                damage: None,
                healing: Some(50),
            },
            ItemType::Sword => ItemDescriptor {
                durability: SWORD_DURABILITY,
                damage: Some(SWORD_DAMAGE),
                healing: None,
            },
            ItemType::Shotgun => ItemDescriptor {
                durability: SHOTGUN_DURABILITY,
                damage: Some(SHOTGUN_DAMAGE),
                healing: None,
            },
        }
    }
}
