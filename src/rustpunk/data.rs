use sqlite;

pub struct GameData {
    connection: Connection,
}

impl GameData {
    pub fn new() {
        let conn = sqlite::open("data/data.db").unwrap();
        GameData {
            connection: conn,
        }
    }

    pub fn load_tile("")
}
