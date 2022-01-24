use crate::{
    config::db::Pool,
    utils::auth_error::AuthError,
    enums::role::Role,
    models::auth::{LoginDTO, Token},
    repository::user_repository,
    utils::token_util
};
use actix_web::web;
use bcrypt::{verify};

#[derive(Serialize, Deserialize)]
struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub role: String
}

pub fn login(login: LoginDTO, pool: &web::Data<Pool>) -> Result<Token, AuthError> {
    info!("Attempt to login...");
    
    if let Ok(staff) = user_repository::get_staff_by_email(&login.email, &pool.get().unwrap()) {
        return verify_user(&login, &User{id: staff.id, email: staff.email, password: staff.password, role: staff.role});
        
    } else if let Ok(admin) = user_repository::get_admin_by_email(&login.email, &pool.get().unwrap()) {

        return verify_user(&login, &User{id: admin.id, email: admin.admin_email, password: admin.password, role: admin.role});
    }
    info!("{}",format!("Unsuccessfull login, wrong credentials - email {}", &login.email));

    Err(AuthError::NotFound(format!("User with email {} does not exist!",&login.email)))
}

fn verify_user(login: &LoginDTO, user: &User) -> Result<Token, AuthError>{
    if verify(&login.password, &user.password).unwrap() {
        let role = Role::from_str(user.role.as_str());
        info!("{}", format!("Successfull login with email {}", &login.email));

        return Ok(Token {accessToken: token_util::create_token(user.id, (*user.email).to_owned(), role)});

    } else {
        info!("{}", format!("Unsuccessfull login, wrong password for email {}", &login.email));

        return Err(AuthError::GenericError(String::from("Password not correct!Try again!")));
    }
}