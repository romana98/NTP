use crate::{
    config::db::Pool,
    models::{ 
        soft_constraint::{SoftConstraintsDTO},
        prefers::{NewPrefers}
    },
    repository::{soft_constraints_repository, prefers_repository}
};
use actix_web::{web};
use diesel::result::Error;
use std::collections::HashMap;

pub fn get_soft_constraints(id: i32, pool: &web::Data<Pool>) -> Result<SoftConstraintsDTO, Error> {
    info!("{}", format!("   Getting soft_constraints {}", id));
  
    let connection = pool.get().expect("Connection from pool");

    let result = soft_constraints_repository::get_soft_constraints(id, &connection);

    let prefers = prefers_repository::get_prefers(id, &connection).unwrap();
    let mut prefers_map = HashMap::new();

    for prefer in prefers {
        prefers_map.insert(prefer.day, prefer.num);
    }

    match result {
        Ok(res) => Ok(SoftConstraintsDTO{
            id: res.id.to_string(),
            prefers: prefers_map,
        }),
        Err(e) => Err(e),
    }
}

pub fn update_soft_constraints(sc_dto: SoftConstraintsDTO, pool: &web::Data<Pool>) -> Result<SoftConstraintsDTO, Error> {
    info!("{}", format!("   Update soft_constraints {}", sc_dto.id));
  
    let connection = pool.get().expect("Connection from pool");

    let mut prefers_vec = Vec::new();
    for (key, value) in sc_dto.prefers {
        let prefer = NewPrefers{
            soft_c_id: sc_dto.id.parse::<i32>().unwrap(),
            day: key,
            num: value
        };
        prefers_vec.push(prefer);
    }

    let result = prefers_repository::update_prefers(sc_dto.id.parse::<i32>().unwrap(), prefers_vec, &connection);

    match result {
        Ok(res) => {

            let mut prefers_map = HashMap::new();
            for prefer in res {
                prefers_map.insert(prefer.day, prefer.num);
            }

            Ok(SoftConstraintsDTO{
                id: sc_dto.id,
                prefers: prefers_map,
            })     
        },
        Err(e) => Err(e),
    }
}
