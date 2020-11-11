
#[derive(Clone, Debug)]
pub struct Inventory {
    pub items: Vec<Box<Item>>,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            items: vec![],
        }
    }

    pub fn add_item(&mut self, item: Box<Item>) {
        self.items.push(item);
    }

    pub fn remove_item(&mut self, idx: i32) -> Box<Item> {
        self.items.remove(idx as usize)
    }

    pub fn get_item(&self, idx: i32) -> &Box<Item> {
        &self.items[idx as usize]
    }
}

#[derive(Clone, Debug)]
pub enum ItemEffect {
    ChangeHealth(i32),
    Message(String),
}

#[derive(Clone, Debug)]
pub enum WearLoc {
    Head,
    Torso,
    Legs,
    Shoulders,
    Hands,
    Hand,
}

#[derive(Clone, Debug)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub on_use: Vec<ItemEffect>,
    pub wearable: Vec<WearLoc>,
    pub consumable: bool,
}

impl Item {
    pub fn new(name: String, description: String) -> Self {
        Item {
            name: name,
            description: description,
            on_use: vec![],
            wearable: vec![],
            consumable: false,
        }
    }

    pub fn healing_potion() -> Self {
        Item {
            name: "healing potion".to_string(),
            description: "It's a healing potion!".to_string(),
            on_use: vec![ItemEffect::ChangeHealth(3)],
            wearable: vec![],
            consumable: true,
        }
    }
}
