use crate::enums::role::Role;
use lazy_static::lazy_static;
use chrono::{Duration, Local};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, EncodingKey, Header};

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

pub fn create_token(id: i32, email: String, role: Role) -> String {
    info!("{}", format!("Creating token for user with email {}", &email));
    let exp_time = Local::now() + Duration::minutes(3600);

    let claims = Claims {
        id: id,
        email: email,
        exp: exp_time.timestamp(),
        role: role.to_string(),
        permissions: vec![role.to_string()],
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
    .expect("Can't create token")
}