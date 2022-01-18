use crate::{
    config::db::Pool,
    models::lecture::{LectureDTO, Lecture, NewLecture},
    utils::{response_util},
    repository::lecture_repository,
};
use actix_web::{web, Error as ActixError};
use diesel::result::Error;

pub fn create_lecture(lecture_dto: LectureDTO, pool: &web::Data<Pool>) -> Result<LectureDTO, Error> {
    info!("{}", format!("   Inserting lecture {}", &lecture_dto.name));
    let lecture = NewLecture {
        name: lecture_dto.name,
        number_of_times: lecture_dto.number_of_times,
    };
    let connection = pool.get().expect("Connection from pool");

    let result = lecture_repository::create_lecture(lecture, &connection);

    match result {
        Ok(res) => Ok(LectureDTO{
            id: res.id.to_string(),
            name: res.name,
            number_of_times: res.number_of_times
        }),
        Err(e) => Err(e),
    }
}

pub fn get_lecture(id: i32, pool: &web::Data<Pool>) -> Result<LectureDTO, Error> {
    info!("{}", format!("   Getting lecture {}", id));
  
    let connection = pool.get().expect("Connection from pool");

    let result = lecture_repository::get_lecture(id, &connection);

    match result {
        Ok(res) => Ok(LectureDTO{
            id: res.id.to_string(),
            name: res.name,
            number_of_times: res.number_of_times
        }),
        Err(e) => Err(e),
    }
}

pub fn delete_lecture(id: i32, pool: &web::Data<Pool>) -> Result<String, ActixError> {
    info!("{}", format!("   Deleting lecture {}", id));

    let connection = pool.get().expect("Connection from pool");
    
    return lecture_repository::delete_lecture(id, &connection)
        .map(|_| "   Lecture successfully deleted!".to_string())
        .map_err(|error| response_util::error_response(error));
}

pub fn update_lecture(lecture_dto: LectureDTO, pool: &web::Data<Pool>) -> Result<LectureDTO, Error> {
    info!("{}", format!("   Update lecture {}", lecture_dto.id));
  
    let connection = pool.get().expect("Connection from pool");
    let lecture = Lecture {
        id: lecture_dto.id.parse::<i32>().unwrap(),
        name: lecture_dto.name,
        number_of_times: lecture_dto.number_of_times,
    };

    let result = lecture_repository::update_lecture(lecture_dto.id.parse::<i32>().unwrap(), lecture, &connection);

    match result {
        Ok(res) => Ok(LectureDTO{
            id: res.id.to_string(),
            name: res.name,
            number_of_times: res.number_of_times
        }),
        Err(e) => Err(e),
    }
}

pub fn get_all_lectures(pool: &web::Data<Pool>) -> Result<Vec<LectureDTO>, Error> {
    info!("   Get all lectures");
  
    let connection = pool.get().expect("Connection from pool");

    let result = lecture_repository::get_all_lectures(&connection);

    match result {
        Ok(res) => {
        let mut vec_lectures = Vec::new();
        
        for lec in res {
            let dto = LectureDTO{
                id: lec.id.to_string(),
                name: lec.name,
                number_of_times: lec.number_of_times
            };
            vec_lectures.push(dto);
        }
        Ok(vec_lectures)},
        Err(e) => Err(e),
    }
}

pub fn get_all_lectures_by_ids(ids: Vec<String>, pool: &web::Data<Pool>) -> Result<Vec<LectureDTO>, Error> {
    info!("   Get all lectures by ids");
  
    let connection = pool.get().expect("Connection from pool");

    let ids_int: Vec<i32> = ids.iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let result = lecture_repository::get_all_lectures_by_ids(ids_int, &connection);

    match result {
        Ok(res) => {
        let mut vec_lectures = Vec::new();
        
        for lec in res {
            let dto = LectureDTO{
                id: lec.id.to_string(),
                name: lec.name,
                number_of_times: lec.number_of_times
            };
            vec_lectures.push(dto);
        }
        Ok(vec_lectures)},
        Err(e) => Err(e),
    }
}