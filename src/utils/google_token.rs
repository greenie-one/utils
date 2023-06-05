use std::fs;
use std::io::Error as IOError;
use serde::{Deserialize, Serialize};
use serde_json::{Error as JSONError};
use std::time::{SystemTime, UNIX_EPOCH, SystemTimeError};
use jsonwebtoken::{encode, Algorithm, Header, EncodingKey};
use jsonwebtoken::errors::Error as JWTError;
use reqwest::{Error as APIError};

#[derive(Debug)]
pub enum Error {
    IO(IOError),
    JSON(JSONError),
    SystemTime(SystemTimeError),
    JWT(JWTError),
    API(APIError),
}

#[derive(Debug, Deserialize, Clone)]
pub struct ClientSecret {
    #[serde(rename = "type")]
    pub secret_type: String,
    pub project_id: String,
    pub private_key_id: String,
    pub private_key: String,
    pub client_email: String,
    pub client_id: String,
    pub auth_uri: String,
    pub token_uri: String,
    pub auth_provider_x509_cert_url: String,
    pub client_x509_cert_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    scope: String,
    aud: String,
    exp: u64,
    iat: u64,
}

#[derive(Debug, Deserialize)]
pub struct APIResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

impl From<IOError> for Error {
    fn from(err: IOError) -> Error {
        Error::IO(err)
    }
}

impl From<JSONError> for Error {
    fn from(err: JSONError) -> Error {
        Error::JSON(err)
    }
}

impl From<SystemTimeError> for Error {
    fn from(err: SystemTimeError) -> Error {
        Error::SystemTime(err)
    }
}

impl From<JWTError> for Error {
    fn from(err: JWTError) -> Error {
        Error::JWT(err)
    }
}

impl From<APIError> for Error {
    fn from(err: APIError) -> Error {
        Error::API(err)
    }
}

pub async fn get_access_token(client_secret: &ClientSecret, scope: String) -> Result<String, Error> {
    let jwt = create_jwt_token(client_secret, scope)?;
    let body = vec![
        ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
        ("assertion", &jwt),
    ];
    let response = reqwest::Client::new()
        .post(client_secret.token_uri.as_str())
        .form(&body)
        .send()
        .await?
        .json::<APIResponse>()
        .await?;
    Ok(response.access_token)
}

pub fn read_secret_from_file(path: String) -> Result<ClientSecret, Error> {
    let client_secret_json: String = fs::read_to_string(path)?;
    let client_secret: ClientSecret = serde_json::from_str(&client_secret_json)?;
    Ok(client_secret)
}

fn create_jwt_token(client_secret: &ClientSecret, scope: String) -> Result<String, Error> {
    let claims = create_jwt_claims(client_secret, scope)?;
    let encoding_key = EncodingKey::from_rsa_pem(client_secret.private_key.as_bytes())?;
    let token = encode(&Header::new(Algorithm::RS256), &claims, &encoding_key)?;
    Ok(token)
}

fn create_jwt_claims(client_secret: &ClientSecret, scope: String) -> Result<Claims, Error> {
    let issue_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)?.as_secs();
    let expiration_time = issue_time + 3600;

    Ok(Claims {
        iss: client_secret.client_email.clone(),
        scope,
        aud: client_secret.token_uri.clone(),
        exp: expiration_time,
        iat: issue_time,
    })
}

#[derive(Debug, Clone)]
pub struct TokenHandler {
    client_secret: ClientSecret,
    scope: String,
}

impl TokenHandler {
    pub fn new(path: String, scope: String) -> Result<TokenHandler, Error> {
        let client_secret = read_secret_from_file(path)?;
        Ok(TokenHandler {
            client_secret,
            scope,
        })
    }

    pub async fn get_access_token(&self) -> Result<String, Error> {
        get_access_token(&self.client_secret, self.scope.clone()).await
    }
}