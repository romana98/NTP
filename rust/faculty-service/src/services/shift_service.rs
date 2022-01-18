use crate::{
    config::db::Pool,
    models::shift::{ShiftDTO, Shift, NewShift},
    utils::{response_util},
    repository::{shift_repository, shifts_faculty_repository},
};
use actix_web::{web, Error as ActixError};
use diesel::result::Error;

pub fn create_shift(shift_dto: ShiftDTO, pool: &web::Data<Pool>) -> Result<ShiftDTO, Error> {
    info!("{}", format!("   Inserting shift"));
    let shift = NewShift {
        start_shift: shift_dto.start,
        end_shift: shift_dto.end,
        day: shift_dto.day
    };
    let connection = pool.get().expect("Connection from pool");

    let result = shift_repository::create_shift(shift, &connection);

    match result {
        Ok(res) => Ok(ShiftDTO{
            id: res.id.to_string(),
            start: res.start_shift,
            end: res.end_shift,
            day: res.day
        }),
        Err(e) => Err(e),
    }
}

pub fn get_shift(id: i32, pool: &web::Data<Pool>) -> Result<ShiftDTO, Error> {
    info!("{}", format!("   Getting shift {}", id));
  
    let connection = pool.get().expect("Connection from pool");

    let result = shift_repository::get_shift(id, &connection);

    match result {
        Ok(res) => Ok(ShiftDTO{
            id: res.id.to_string(),
            start: res.start_shift,
            end: res.end_shift,
            day: res.day
        }),
        Err(e) => Err(e),
    }
}

pub fn delete_shift(id: i32, pool: &web::Data<Pool>) -> Result<String, ActixError> {
    info!("{}", format!("   Deleting shift {}", id));

    let connection = pool.get().expect("Connection from pool");

    let resp = shifts_faculty_repository::delete_shifts_faculty_by_shift_id(id, &connection);
    if resp.is_err(){
        return Err(response_util::error_response(Error::NotFound))
    }
    
    return shift_repository::delete_shift(id, &connection)
        .map(|_| "   Shift successfully deleted!".to_string())
        .map_err(|error| response_util::error_response(error));
}

pub fn update_shift(shift_dto: ShiftDTO, pool: &web::Data<Pool>) -> Result<ShiftDTO, Error> {
    info!("{}", format!("   Update shift {}", shift_dto.id));
  
    let connection = pool.get().expect("Connection from pool");
    let shift = Shift {
        id: shift_dto.id.parse::<i32>().unwrap(),
        start_shift: shift_dto.start,
        end_shift: shift_dto.end,
        day: shift_dto.day
    };

    let result = shift_repository::update_shift(shift_dto.id.parse::<i32>().unwrap(), shift, &connection);

    match result {
        Ok(res) => Ok(ShiftDTO{
            id: res.id.to_string(),
            start: res.start_shift,
            end: res.end_shift,
            day: res.day
        }),
        Err(e) => Err(e),
    }
}

pub fn get_all_shifts(pool: &web::Data<Pool>) -> Result<Vec<ShiftDTO>, Error> {
    info!("   Get all shifts");
  
    let connection = pool.get().expect("Connection from pool");

    let result = shift_repository::get_all_shifts(&connection);

    match result {
        Ok(res) => {
        let mut vec_shifts = Vec::new();
        
        for shift in res {
            let dto = ShiftDTO{
                id: shift.id.to_string(),
                start: shift.start_shift,
                end: shift.end_shift,
                day: shift.day
            };
            vec_shifts.push(dto);
        }
        Ok(vec_shifts)},
        Err(e) => Err(e),
    }
}