use chrono;
use jsonwebtoken as jwt;
use reqwest;

use chrono::{Duration, Utc};
use jwt::{Algorithm, EncodingKey, Header};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use yup_oauth2::{InstalledFlowAuthenticator, InstalledFlowReturnMethod};
use std::fs;

use crate::Error;

// Service Account Credentials, Json format
const JSON_FILENAME: &str =
    "D:\\Projects\\Work\\Greenie\\communications\\keys\\local\\googleapi\\service-account-key.json";

// Permissions to request for Access Token
const SCOPES: &str = "https://www.googleapis.com/auth/gmail.send";

// Set how long this token will be valid in seconds
const EXPIRES_IN: i64 = 3600; // Expires in 1 hour

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: Option<String>,
    sub: Option<String>,
    aud: Option<String>,
    iat: Option<u64>,
    exp: Option<u64>,
    scope: Option<String>,
}

impl Default for Claims {
    fn default() -> Self {
        Claims {
            iss: None,
            sub: None,
            aud: None,
            iat: None,
            exp: None,
            scope: None,
        }
    }
}

#[derive(Clone)]
pub struct TokenHandler {
    creds: serde_json::Value,
}

impl TokenHandler {
    pub fn new() -> Self {
        let json_filename = JSON_FILENAME;
        let creds = TokenHandler::load_json_credentials(&json_filename);
        TokenHandler {
            creds,
        }
    }
}

impl TokenHandler {
    fn load_json_credentials(filename: &str) -> serde_json::Value {
        let data = fs::read_to_string(filename).expect("Failed to read file");
        serde_json::from_str(&data).expect("Failed to parse JSON")
    }
    
    fn load_private_key(json_cred: &serde_json::Value) -> String {
        json_cred["private_key"].as_str().to_owned().unwrap().into()
    }
    
    fn create_signed_jwt(pkey: &str, pkey_id: &str, email: &str, scope: &str) -> String {
        let now = Utc::now();
        let expires = now + Duration::seconds(EXPIRES_IN);
    
        let header = Header {
            kid: Some(pkey_id.to_owned()),
            alg: Algorithm::RS256,
            typ: Some("JWT".to_owned()),
            ..Default::default()
        };
    
        let claims = Claims {
            iss: Some(email.to_owned()),
            sub: Some(email.to_owned()),
            aud: Some("https://oauth2.googleapis.com/token".to_owned()),
            iat: Some(now.timestamp() as u64),
            exp: Some(expires.timestamp() as u64),
            scope: Some(scope.to_owned()),
        };
    
        let key = EncodingKey::from_rsa_pem(pkey.as_ref()).unwrap();
        jwt::encode(&header, &claims, &key).unwrap()
    }
    
    async fn exchange_jwt_for_access_token(signed_jwt: &str) -> Result<String, String> {
        let auth_url = "https://oauth2.googleapis.com/token";
    
        let client = Client::new();
        let params = [
            ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
            ("assertion", signed_jwt),
        ];
    
        let res = client
            .post(auth_url)
            .form(&params)
            .send()
            .await
            .map_err(|e| format!("HTTP request failed: {}", e))?;
    
        if res.status().is_success() {
            let body = res
                .text()
                .await
                .map_err(|e| format!("Failed to read response body: {}", e))?;
            let json: serde_json::Value =
                serde_json::from_str(&body).map_err(|e| format!("Failed to parse JSON: {}", e))?;

            println!("json: {:?}\n", json);
    
            if let Some(access_token) = json["access_token"].as_str() {
                Ok(access_token.to_owned())
            } else {
                Err("Access token not found in response".to_owned())
            }
        } else {
            let body = res
                .text()
                .await
                .map_err(|e| format!("Failed to read response body: {}", e))?;
            Err(body.into())
        }
    }
}

impl TokenHandler {
    pub async fn get_access_token(&self) -> Result<String, Error> {
        let cred = &self.creds;
        let private_key = TokenHandler::load_private_key(&cred);
        let s_jwt = TokenHandler::create_signed_jwt(
            private_key.as_str(),
            &cred["private_key_id"].as_str().unwrap(),
            &cred["client_email"].as_str().unwrap(),
            SCOPES,
        );
    
        match TokenHandler::exchange_jwt_for_access_token(&s_jwt).await {
            Ok(token) => {
                println!("Access Token: {}\n", token);
                Ok(token)
            }
            Err(err) => {
                println!("Error: {}", err);
                Err(Error::GoogleAccessTokenError)
            }
        }
    }
    
    pub async fn get_signed_jwt(&self) -> String {
        let cred = &self.creds;
        let private_key = TokenHandler::load_private_key(&cred);
        let s_jwt = TokenHandler::create_signed_jwt(
            private_key.as_str(),
            &cred["private_key_id"].as_str().unwrap(),
            &cred["client_email"].as_str().unwrap(),
            SCOPES,
        );
    
        s_jwt
    }
}

pub async fn test() {
    // Read application secret from a file. Sometimes it's easier to compile it directly into
    // the binary. The clientsecret file contains JSON like `{"installed":{"client_id": ... }}`
    let secret = yup_oauth2::read_service_account_key(JSON_FILENAME)
        .await.unwrap();

    // Create an authenticator that uses an InstalledFlow to authenticate. The
    // authentication tokens are persisted to a file named tokencache.json. The
    // authenticator takes care of caching tokens to disk and refreshing tokens once
    // they've expired.
    let auth = InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
    .persist_tokens_to_disk("tokencache.json")
    .build()
    .await
    .unwrap();

    let scopes = &[SCOPES];

    // token(<scopes>) is the one important function of this crate; it does everything to
    // obtain a token that can be sent e.g. as Bearer token.
    match auth.token(scopes).await {
        Ok(token) => println!("The token is {:?}", token),
        Err(e) => println!("error: {:?}", e),
    }
}
