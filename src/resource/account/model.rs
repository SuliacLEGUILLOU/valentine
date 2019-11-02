use r2d2_postgres::PostgresConnectionManager;
type Pool = r2d2::PooledConnection<PostgresConnectionManager>;

use uuid::Uuid;

use crate::engine::password_engine::hash_password;

// TODO: Have a basic presenter
#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    #[serde(default = "generate_id")]   // TODO Move that to a more generic place
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String
}

fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

// TODO! Turn request into async
impl Model {
    pub fn get_by_id(conn: Pool, id: &str) -> Vec<Model> {
        let mut accounts: Vec<Model> = Vec::with_capacity(1);

        for row in &conn.query("SELECT id, name, email, password FROM account WHERE id=$1", &[&id]).unwrap() {
            accounts.push(Model {
                id: row.get(0),
                name: row.get(1),
                email: row.get(2),
                password: row.get(3),
            })
        }
        return accounts
    }

    pub fn get_all(conn: Pool) -> Vec<Model> {
        let mut accounts: Vec<Model> = Vec::with_capacity(100);

        for row in &conn.query("SELECT id, name, email, password FROM account LIMIT 100", &[]).unwrap() {
            accounts.push(Model {
                id: row.get(0),
                name: row.get(1),
                email: row.get(2),
                password: row.get(3),
            })
        }
        return accounts
    }

    pub fn insert(&self, conn: Pool) {
        let hash = hash_password(&self.password).unwrap();

        conn.execute("INSERT INTO account (id, name, email, password) VALUES ($1, $2, $3, $4)", &[&self.id, &self.name, &self.email, &hash]).unwrap();
    }

    pub fn patch(&self, conn: Pool) {
        conn.execute("UPDATE account SET name=$1, email=$2, password=$3 WHERE id=$4", &[&self.name, &self.email, &self.password, &self.id]).unwrap();
    }

    pub fn delete(&self, conn: Pool) {
        conn.execute("DELETE FROM account WHERE id=$1", &[&self.id]).unwrap();
    }
}