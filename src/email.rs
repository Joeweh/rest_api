use reqwest::{Client, Error, Response};

pub struct EmailClient {
    key: String,
    client: Client
}

impl EmailClient {
    pub fn new(apikey: String, client: Client) -> Self {
        EmailClient { key: apikey, client }
    }

    pub async fn send_email(&self) -> Result<Response, Error> {
        self.client
            .post("https://api.sendinblue.com/v3/smtp/email")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("api-key", &self.key)
            .body("{\"sender\":{\"name\":\"Joseph Siracusa\",\"email\":\"joeyqsa211@gmail.com\"},\"to\":[{\"email\":\"joeyqsa@outlook.com\",\"name\":\"Joeweh\"}],\"textContent\":\"This is a test email\",\"subject\":\"Test Email\"}")
            .send()
            .await
    }
}