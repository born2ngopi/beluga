// https://docs.rs/postgres/latest/postgres/struct.Client.html
use postgres::{Client, NoTls};

pub struct PostgresClient {
    client: Client,
}

impl PostgresClient {
    pub fn new(url: &str) -> Result<Self, postgres::Error> {
        let client = Client::connect(url, NoTls)?;
        Ok(Self { client })
    }

    pub fn query(&mut self, query: &str) -> Result<Vec<postgres::Row>, postgres::Error> {
        let rows = self.client.query(query, &[])?;
        Ok(rows)
    }
}