#[derive(Serialize, Deserialize)]
pub struct LoginDTO {
    pub email: String,
    pub password: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Token {
    pub accessToken: String,
}