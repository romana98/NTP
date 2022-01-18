use crate::{
    config::db::Pool,
    models::hard_constraints::{HardConstraintDTO, HardConstraint, NewHardConstraint},
    utils::{response_util},
    repository::hard_constraints_repository,
};
use actix_web::{web, Error as ActixError};
use diesel::result::Error;

pub fn create_hard_constraint(hard_constraint_dto: HardConstraintDTO, pool: &web::Data<Pool>) -> Result<HardConstraintDTO, Error> {
    info!("{}", format!("   Inserting hard_constraint"));
    let hard_constraint = NewHardConstraint {
        daily_max: hard_constraint_dto.daily_max,
        weekly_max: hard_constraint_dto.weekly_max,
        weekly_min: hard_constraint_dto.weekly_min,
        max_per_shift: hard_constraint_dto.max_per_shift
    };
    let connection = pool.get().expect("Connection from pool");

    let result = hard_constraints_repository::create_hard_constraint(hard_constraint, &connection);

    match result {
        Ok(res) => Ok(HardConstraintDTO {
            id: res.id.to_string(),
            daily_max: res.daily_max,
            weekly_max: res.weekly_max,
            weekly_min: res.weekly_min,
            max_per_shift: res.max_per_shift
        }),
        Err(e) => Err(e),
    }
}

pub fn get_hard_constraint(id: i32, pool: &web::Data<Pool>) -> Result<HardConstraintDTO, Error> {
    info!("{}", format!("   Getting hard_constraint {}", id));
  
    let connection = pool.get().expect("Connection from pool");

    let result = hard_constraints_repository::get_hard_constraint(id, &connection);

    match result {
        Ok(res) => Ok(HardConstraintDTO{
            id: res.id.to_string(),
            daily_max: res.daily_max,
            weekly_max: res.weekly_max,
            weekly_min: res.weekly_min,
            max_per_shift: res.max_per_shift
        }),
        Err(e) => Err(e),
    }
}

pub fn delete_hard_constraint(id: i32, pool: &web::Data<Pool>) -> Result<String, ActixError> {
    info!("{}", format!("   Deleting hard_constraint {}", id));

    let connection = pool.get().expect("Connection from pool");
    
    return hard_constraints_repository::delete_hard_constraint(id, &connection)
        .map(|_| "   Hard_constraint successfully deleted!".to_string())
        .map_err(|error| response_util::error_response(error));
}

pub fn update_hard_constraint(hard_constraint_dto: HardConstraintDTO, pool: &web::Data<Pool>) -> Result<HardConstraintDTO, Error> {
    info!("{}", format!("   Update hard_constraint {}", hard_constraint_dto.id));
  
    let connection = pool.get().expect("Connection from pool");
    let hard_constraint = HardConstraint {
            id: hard_constraint_dto.id.parse::<i32>().unwrap(),
            daily_max: hard_constraint_dto.daily_max,
            weekly_max: hard_constraint_dto.weekly_max,
            weekly_min: hard_constraint_dto.weekly_min,
            max_per_shift: hard_constraint_dto.max_per_shift
    };

    let result = hard_constraints_repository::update_hard_constraint(hard_constraint_dto.id.parse::<i32>().unwrap(), hard_constraint, &connection);

    match result {
        Ok(res) => Ok(HardConstraintDTO{
            id: res.id.to_string(),
            daily_max: res.daily_max,
            weekly_max: res.weekly_max,
            weekly_min: res.weekly_min,
            max_per_shift: res.max_per_shift
        }),
        Err(e) => Err(e),
    }
}

pub fn get_all_hard_constraints(pool: &web::Data<Pool>) -> Result<Vec<HardConstraintDTO>, Error> {
    info!("   Get all hard_constraints");
  
    let connection = pool.get().expect("Connection from pool");

    let result = hard_constraints_repository::get_all_hard_constraints(&connection);

    match result {
        Ok(res) => {
        let mut vec_hard_constraints = Vec::new();
        
        for hard_constraint in res {
            let dto = HardConstraintDTO{
                id: hard_constraint.id.to_string(),
                daily_max: hard_constraint.daily_max,
                weekly_max: hard_constraint.weekly_max,
                weekly_min: hard_constraint.weekly_min,
                max_per_shift: hard_constraint.max_per_shift
            };
            vec_hard_constraints.push(dto);
        }
        Ok(vec_hard_constraints)},
        Err(e) => Err(e),
    }
}