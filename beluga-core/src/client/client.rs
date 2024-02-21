use crate::{
    mysql::Client as MysqlClient,
    postgres::PostgresClient,
    trino::TrinoClient,
};
use rustc_hash::HashMap;

struct connection{
    driver: &Str,
    mysqlClient: MysqlClient,
    postgresClient: PostgresClient,
    trinoClient: TrinoClient,
}

pub struct Client{
    // HashMap<connectionName, connection>
    connections: HashMap<String, connection>,
}

impl Client {
    pub fn new() -> Result<Self, Error> {
        Ok(Self { connections: HashMap::new() })
    }

    pub fn addConnection(&mut self, connectionName: &str, driver: &str, url: &str, auth: Auth, ssl: Ssl) -> Result<(), Error> {
        match driver {
            "mysql" => {
                let mysqlClient = MysqlClient::new(url)?;
                self.connections.insert(connectionName, connection{driver, mysqlClient, postgresClient: None, trinoClient: None});
            },
            "postgres" => {
                let postgresClient = PostgresClient::new(url)?;
                self.connections.insert(connectionName, connection{driver, mysqlClient: None, postgresClient, trinoClient: None});
            },
            "trino" => {
                let trinoClient = TrinoClient::new(url, auth, ssl)?;
                self.connections.insert(connectionName, connection{driver, mysqlClient: None, postgresClient: None, trinoClient});
            },
            _ => {
                return Err(Error::new("Invalid driver"));
            }
        }
        Ok(())
    }

    pub fn query(&self, connectionName: &str, query: &str) -> Result<Vec<Row>, Error> {
        let conn = self.connections.get(connectionName);
        match conn.driver {
            "mysql" => {
                let rows = conn.mysqlClient.query(query).await?;
                Ok(rows)
            },
            "postgres" => {
                let rows = conn.postgresClient.query(query)?;
                Ok(rows)
            },
            "trino" => {
                let rows = conn.trinoClient.query(query).await?;
                Ok(rows)
            },
            _ => {
                return Err(Error::new("Invalid driver"));
            }
        }
    }
}