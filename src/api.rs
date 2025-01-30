use crate::types::ResyKeys;
use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_json::Value;
use url::Url;

pub struct ResyApi {
    client: reqwest::Client,
    keys: ResyKeys,
}

impl ResyApi {
    pub fn new(keys: ResyKeys) -> Self {
        Self {
            client: reqwest::Client::new(),
            keys,
        }
    }

    pub async fn get_reservations(
        &self,
        date: &str,
        party_size: i32,
        venue_id: i32,
    ) -> Result<Value> {
        let mut url = Url::parse("whaturlisthis")?;

        url.query_pairs_mut();

        let mut headers = HeaderMap::new();

        let response = self
            .client
            .get(url)
            .headers(headers)
            .send()
            .await?
            .error_for_status()?;

        let json = response.json().await?;
        Ok(json)
    }

    pub async fn get_reservation_details(
        &self,
        config_id: &str,
        date: &str,
        party_size: i32,
    ) -> Result<Value> {
        let mut url = Url::parse("thisurlseemsimportant")?;

        url.query_pairs_mut()
            .append_pair("config_id", config_id)
            .append_pair("day", date)
            .append_pair("party_size", &party_size.to_string());

        let response = self
            .client
            .get(url)
            .headers(self.create_headers(false)?)
            .send()
            .await?
            .error_for_status()?;
        let json = response.json().await?;
        Ok(json)
    }

    pub async fn post_reservation(
        &self,
        payment_method_id: i32,
        book_token: &str,
    ) -> Result<Value> {
        let url = Url::parse("thenameofthisurlissecret")?;

        let form_data = [
            ("book_token", book_token.to_string()),
            (
                "struct_payment_method",
                format!("{{\"id\":{}}}", payment_method_id),
            ),
        ];

        let response = self
            .client
            .post(url)
            .headers(self.create_headers(true)?)
            .form(&form_data)
            .send()
            .await?
            .error_for_status()?;

        let json = response.json().await?;
        Ok(json)
    }

    fn create_headers(&self, is_post: bool) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "User-Agent",
            HeaderValue::from_static(
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36...",
            ),
        );
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json, text/plain, */*"),
        );
        headers.insert(
            "Accept-Language",
            HeaderValue::from_static("en-US,en;q=0.9"),
        );
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("ResyAPI api_key=\"{}\"", self.keys.api_key))?,
        );
        headers.insert(
            "x-resy-auth-token",
            HeaderValue::from_str(&self.keys.auth_token)?,
        );

        if is_post {
            headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            );
            headers.insert(
                "Origin",
                HeaderValue::from_static("https://widgets.resy.com"),
            );
            headers.insert(
                "Referer",
                HeaderValue::from_static("https://widgets.resy.com/"),
            );
        }

        Ok(headers)
    }
}
