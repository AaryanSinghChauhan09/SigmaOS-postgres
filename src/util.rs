//! postgres model — PostgreSQL database server/client.

#[derive(Debug, Clone)]
pub struct Database {
    pub name: String,
    pub owner: String,
    pub encoding: String,
}

#[derive(Debug, Clone)]
pub struct Table {
    pub name: String,
    pub database: String,
    pub rows: u64,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub databases: Vec<Database>,
    pub tables: Vec<Table>,
}

impl Default for Model {
    fn default() -> Self { Self { databases: vec![
        Database { name: "postgres".into(), owner: "postgres".into(), encoding: "UTF8".into() },
    ], tables: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn createdb(&mut self, name: &str, owner: &str) -> bool {
        if self.databases.iter().any(|d| d.name == name) { return false; }
        self.databases.push(Database { name: name.into(), owner: owner.into(), encoding: "UTF8".into() });
        true
    }

    pub fn dropdb(&mut self, name: &str) -> bool {
        if let Some(pos) = self.databases.iter().position(|d| d.name == name) { self.databases.remove(pos); true } else { false }
    }

    pub fn list_databases(&self) -> &[Database] { &self.databases }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn createdb_dropdb() {
        let mut p = Model::new();
        assert!(p.createdb("appdb", "app"));
        assert!(!p.createdb("appdb", "app"));
        assert!(p.dropdb("appdb"));
    }
}
