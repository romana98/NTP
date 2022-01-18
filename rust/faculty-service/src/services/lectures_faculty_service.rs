use crate::{
    config::db::Pool,
    utils::{response_util},
    repository::{lectures_faculty_repository},
};
use actix_web::{web, Error as ActixError};

pub fn delete_lectures_faculty(id: i32, pool: &web::Data<Pool>) -> Result<String, ActixError> {
    info!("{}", format!("   Deleting lectures_faculty {}", id));

    let connection = pool.get().expect("Connection from pool");

    return lectures_faculty_repository::delete_lectures_faculty_by_lecture(id, &connection)
        .map(|_| "   Lectures_faculty successfully deleted!".to_string())
        .map_err(|error| response_util::error_response(error));
}