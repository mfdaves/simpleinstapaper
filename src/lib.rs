use reqwest::Client;
use std::env;

pub static BASE_URL: &str = "https://www.instapaper.com/api";

pub struct InstapaperSimpleClient {
    pub client: Client,
    pub username: String,
    pub password: String,
}

impl InstapaperSimpleClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            username: env::var("INSTAPAPER_USERNAME")
                .expect("Username is mandatory. Check your env var"),
            password: env::var("INSTAPAPER_PASSWORD")
                .expect("Password is mandatory. Check your env var."),
        }
    }

    pub fn with_credentials(username: String, password: String) -> Self {
        Self {
            client: Client::new(),
            username,
            password,
        }
    }

    pub async fn auth(&self) -> Result<bool, reqwest::Error> {
        Ok(self
            .client
            .get(format!("{BASE_URL}/authenticate"))
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await?
            .status()
            .is_success())
    }

    pub async fn add<I: Into<String>>(&self, url: I) -> Result<bool, reqwest::Error> {
        Ok(self
            .client
            .get(format!("{BASE_URL}/add"))
            .basic_auth(&self.username, Some(&self.password))
            .query(&[("url", &url.into().as_str())])
            .send()
            .await?
            .status()
            .is_success())
    }
}
