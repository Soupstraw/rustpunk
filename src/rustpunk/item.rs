
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
}

#[derive(Clone, Debug)]
pub struct Item {
    pub name: String,
    pub description: String,
}

impl Item {
    pub fn new(name: String, description: String) -> Self {
        Item {
            name: name,
            description: description,
        }
    }
}
