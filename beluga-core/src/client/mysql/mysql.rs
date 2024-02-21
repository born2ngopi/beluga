use mysql::*;
use mysql::prelude::*;

pub struct Client {
    pool: Pool,
}

impl Client {
    pub fn new(url: &str) -> Result<Self, Error> {
        let pool = Pool::new(url)?;
        Ok(Self { pool })
    }

    pub async fn query(&self, query: &str) -> Result<Vec<Row>, Error> {
        let mut conn = self.pool.get_conn().await?;
        let rows = conn.query(query).await?;
        Ok(rows)
    }
}
