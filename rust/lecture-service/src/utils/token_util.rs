use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use actix_web_grants::permissions::AttachPermissions;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web::{Error, dev::ServiceRequest};
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};

lazy_static! {
    static ref JWT_SECRET: String = std::env::var("JWT_SECRET").expect("Can't read JWT_SECRET");
}

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub id: i32,
    pub email: String,
    pub exp: i64,
    pub permissions: Vec<String>,
    role: String,
}

pub fn decode_token(token: &str) -> TokenData<Claims> {
    info!("Decoding token...{}", token);
    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default(),
    )
    .expect("Can't decode token")
}

pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    info!("Validator...");

    let token = credentials.token();
    let token_data = decode_token(&token);
    
    req.attach(token_data.claims.permissions);
    Ok(req)
}