use r2d2_postgres::PostgresConnectionManager;
type Pool = r2d2::PooledConnection<PostgresConnectionManager>;

use crate::engine::password_engine::{hash_password, check_password};
use crate::engine::uuid_engine::*;

// TODO: Have a basic presenter
/**
 * Internal account structure
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    #[serde(default = "generate_id")]
    pub id: String,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String
}

impl Model {
    /**
     * Get an account by id
     */
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

    /**
     * Get an account by it's email and password for login purpose
     */
    pub fn get_for_login(conn: Pool, email: String, password: String) -> Model {
        let mut accounts: Vec<Model> = Vec::with_capacity(1);

        for row in &conn.query("SELECT id, name, email, password FROM account WHERE email=$1", &[&email]).unwrap() {
            accounts.push(Model {
                id: row.get(0),
                name: row.get(1),
                email: row.get(2),
                password: row.get(3),
            })
        }
        if accounts.len() != 1 {
            panic!("Catch this and throw 403");
        }
        let account = accounts.pop().unwrap();
        if !check_password(&password.to_string(), &account.password) {
            panic!("Catch this and throw 403");
        }
        return account
    }

    /**
     * Get all account from the database (default limit is 100)
     */
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

    /**
     * Insert an new account in the database
     * Password is secured at this point
     */
    pub fn insert(&mut self, conn: Pool) {
        self.password = hash_password(&self.password);

        conn.execute("INSERT INTO account (id, name, email, password) VALUES ($1, $2, $3, $4)", &[&self.id, &self.name, &self.email, &self.password]).unwrap();
    }

    /**
     * Update an account detail
     */
    pub fn patch(&self, conn: Pool) {
        conn.execute("UPDATE account SET name=$1, email=$2 WHERE id=$3", &[&self.name, &self.email, &self.id]).unwrap();
    }

    /**
     * Delete an account
     */
    pub fn delete(&self, conn: Pool) {
        conn.execute("DELETE FROM account WHERE id=$1", &[&self.id]).unwrap();
    }
}