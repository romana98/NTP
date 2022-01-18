use crate::{
    config::db::Pool,
    utils::{response_util},
    models::staff_faculty::{StaffFacultyDTO, NewStaffFaculty},
    repository::{staff_faculty_repository},
};
use actix_web::{web, Error as ActixError};
use diesel::result::Error;

pub fn create_staff_faculty(staff_faculty_dto: StaffFacultyDTO, pool: &web::Data<Pool>) -> Result<StaffFacultyDTO, Error> {
    info!("{}", format!("   Inserting staff_faculty"));
    let staff_faculty = NewStaffFaculty {
        staff_id: staff_faculty_dto.staff_id,
        faculty_id: staff_faculty_dto.faculty_id
    };
    let connection = pool.get().expect("Connection from pool");

    let result = staff_faculty_repository::create_staff_faculty(staff_faculty, &connection);

    match result {
        Ok(res) => Ok(StaffFacultyDTO {
            id: res.id,
            staff_id: res.staff_id,
            faculty_id: res.faculty_id
        }),
        Err(e) => Err(e),
    }
}

pub fn update_staff_faculty(staff_faculty_dto: StaffFacultyDTO, pool: &web::Data<Pool>) -> Result<StaffFacultyDTO, Error> {
    info!("{}", format!("   Inserting hard_constraint"));
    let staff_faculty = NewStaffFaculty {
        staff_id: staff_faculty_dto.staff_id,
        faculty_id: staff_faculty_dto.faculty_id
    };
    let connection = pool.get().expect("Connection from pool");

    let result = staff_faculty_repository::update_staff_faculty(staff_faculty, staff_faculty_dto.staff_id, &connection);
    match result {
        Ok(res) => Ok(StaffFacultyDTO {
            id: res.id,
            staff_id: res.staff_id,
            faculty_id: res.faculty_id
        }),
        Err(e) => Err(e),
    }
}

pub fn delete_staff_faculty(id: i32, pool: &web::Data<Pool>) -> Result<String, ActixError> {
    info!("{}", format!("   Deleting staff_faculty {}", id));

    let connection = pool.get().expect("Connection from pool");

    return staff_faculty_repository::delete_staff_faculty_by_staff(id, &connection)
        .map(|_| "   Staff_faculty successfully deleted!".to_string())
        .map_err(|error| response_util::error_response(error));
}