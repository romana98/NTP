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

pub fn login(login: LoginDTO, pool: &web::Data<Pool>) -> Result<Token, AuthError> {
    info!("Attempt to login...");
    
    if let Ok(staff) = user_repository::get_staff_by_email(&login.email, &pool.get().unwrap()) {

        if verify(login.password, &staff.password).unwrap() {
            let role = Role::from_str(staff.role.as_str());
            info!("{}", format!("Successfull login with email {}", &login.email));

            return Ok(Token {accessToken: token_util::create_token(staff.id, staff.email, role)});

        } else {
            info!("{}", format!("Unsuccessfull login, wrong password for email {}", &login.email));

            return Err(AuthError::GenericError(String::from("Password not correct!Try again!")));
        }
    } else if let Ok(admin) = user_repository::get_admin_by_email(&login.email, &pool.get().unwrap()) {

        if verify(login.password, &admin.password).unwrap() {
            let role = Role::from_str(admin.role.as_str());
            info!("{}", format!("Successfull login with email {}", &login.email));

            return Ok(Token {accessToken: token_util::create_token(admin.id, admin.admin_email, role)});

        } else {
            info!("{}", format!("Unsuccessfull login, wrong password for email {}", &login.email));

            return Err(AuthError::GenericError(String::from("Password not correct!Try again!")));
        }
    }
    info!("{}",format!("Unsuccessfull login, wrong credentials - email {}", &login.email));

    Err(AuthError::NotFound(format!("User with email {} does not exist!",&login.email)))
}