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
