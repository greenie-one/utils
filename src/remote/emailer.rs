use crate::{
    env_config::REMOTE_URL,
    errors::api_errors::{APIError, APIResult}, structs::token_claims::TokenClaims,
};
use std::collections::HashMap;
use tracing::log::error;

#[derive(Clone)]
pub struct Emailer {
    client: reqwest::Client,
}

impl Emailer {
    pub fn new() -> Emailer {
        Emailer {
            client: reqwest::Client::new(),
        }
    }

    async fn send_email(&self, email: &str, subject: &str, messsage: &str) -> APIResult<()> {
        let remote = REMOTE_URL.to_string();
        let mut map = HashMap::new();
        map.insert("email", email);
        map.insert("subject", subject);
        map.insert("message", messsage);
        let res = self
            .client
            .post(format!("{}/emailer/send", remote))
            .json(&map)
            .send()
            .await;

        let res = match res {
            Ok(res) => res,
            Err(err) => {
                error!("{}", err);
                return Err(APIError::InternalServerError(
                    "Email failed to send".to_string(),
                ));
            }
        };

        if res.status().is_success() {
            return Ok(());
        } else {
            error!("{:?}", res);
            return Err(APIError::InternalServerError(
                "Email failed to send".to_string(),
            ));
        }
    }

    pub async fn notify_bulk_upload(&self, user_details: TokenClaims, url: &str) -> APIResult<()> {
        let contact = match user_details.email {
            Some(val) => val,
            None => format!("No email found for {}", user_details.sub),
        };
        let message = format!("File uploaded by {}, url: {}", contact, url);
        self
            .send_email("ratneshjain40@gmail.com", "Bulk Upload", message.as_str())
            .await
            .unwrap();
        Ok(())
    }
}
