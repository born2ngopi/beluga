use prusto::auth::Auth;
use prusto::ssl::Ssl;
use prusto::{ClientBuilder, Row};

pub struct TrinoClient {
    client: prusto::Client,
}

impl TrinoClient {
    pub fn new(url: &str, auth: Auth, ssl: Ssl) -> Result<Self, prusto::Error> {
        let client = ClientBuilder::new(url)
            .auth(auth)
            .ssl(ssl)
            .build()?;
        Ok(Self { client })
    }

    pub async fn query(&self, query: &str) -> Result<Vec<Row>, prusto::Error> {
        let rows = self.client.query(query).await?;
        Ok(rows)
    }
}