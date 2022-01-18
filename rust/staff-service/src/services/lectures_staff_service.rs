use crate::{
    config::db::Pool,
    utils::{response_util},
    repository::{lectures_staff_repository}
};
use actix_web::{web, Error as ActixError};

pub fn delete_lectures_staff(id: i32, pool: &web::Data<Pool>) -> Result<String, ActixError> {
    info!("{}", format!("   Deleting lectures_staff {}", id));

    let connection = pool.get().expect("Connection from pool");
    
    return lectures_staff_repository::delete_lectures_staff_by_lecture(id, &connection)
        .map(|_| "   Lectures_staff successfully deleted!".to_string())
        .map_err(|error| response_util::error_response(error));
}