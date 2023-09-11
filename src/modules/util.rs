use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

// Remove the import of mongodb::bson::oid::ObjectId

#[derive(Debug)]
pub enum EncodeJwtHelper {
    Ok(String),
    Err,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Token {
    pub token: String,
    pub refresh_token: String,
}

#[derive(Debug)]
pub enum DecodeJwtHelper {
    Ok(TokenData<Claims>),
    Err,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: String,
    pub exp: usize,
}

// Update the encode_jwt function to take a user_id of type String
pub fn encode_jwt(user_id: String, secret: &str, expiration: i64) -> EncodeJwtHelper {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(expiration))
        .expect("valid timestamp")
        .timestamp();

    let my_claims = Claims {
        user_id,
        exp: expiration as usize,
    };
    match encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(secret.as_ref()),
    ) {
        Ok(token) => EncodeJwtHelper::Ok(token),
        Err(_) => EncodeJwtHelper::Err,
    }
}

// Remove the decode_jwt function as it's not using ObjectId anymore

pub fn encode_token_and_refresh(
    user_id: String,
    jwt_secret: &'static str,
    refresh_token_secret: &'static str,
    expiration_refresh_token: i64,
    expiration_token: i64,
) -> Result<Token, ()> {
    match encode_jwt(user_id.clone(), jwt_secret, expiration_token) {
        EncodeJwtHelper::Ok(token) => {
            match encode_jwt(user_id, refresh_token_secret, expiration_refresh_token) {
                EncodeJwtHelper::Ok(refresh_token) => Ok(Token {
                    token,
                    refresh_token,
                }),
                EncodeJwtHelper::Err => Err(()),
            }
        }
        EncodeJwtHelper::Err => Err(()),
    }
}
