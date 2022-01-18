use crate::{
    config::db::Pool,
    models::{ 
        staff::{StaffDTO, Staff, NewStaff}, 
        lectures_staffs::{NewLecturesStaff, IdsDTO}
    },
    utils::{response_util},
    repository::{staff_repository, lectures_staff_repository, soft_constraints_repository, prefers_repository},
    enums::role
};
use actix_web::{web, Error as ActixError};
use diesel::result::Error;
use std::vec::Vec;
use bcrypt::{hash, DEFAULT_COST};


pub fn create_staff(staff_dto: StaffDTO, pool: &web::Data<Pool>) -> Result<StaffDTO, Error> {
    //let uid = token_util::get_user_id(req.clone()).unwrap();
    info!("{}", format!("   Inserting staff {} {}", &staff_dto.name, &staff_dto.surname));

    let connection = pool.get().expect("Connection from pool");

    let mut sc_id = 0;

     //add sc
     let res_sc = soft_constraints_repository::create_soft_constraints(&connection);
     if res_sc.is_err(){
        return res_sc.map(|_|  StaffDTO {id: "-1".to_string(), name: "".to_string(), surname: "".to_string(), email: "".to_string(),
                                faculty: "-1".to_string(), lectures: Vec::new()})
                 .map_err(|error| error);
     }else if let Ok(sc) = res_sc.as_ref() {
        
        let res_sc_pref = prefers_repository::create_prefers(sc.id, &connection);
         if res_sc_pref.is_err(){
            return res_sc.map(|_|  StaffDTO {id: "-1".to_string(), name: "".to_string(), surname: "".to_string(), email: "".to_string(),
                               faculty: "-1".to_string(), lectures: Vec::new()})
                 .map_err(|error| error);
         }
        sc_id = sc.id;
     }

    let staff = NewStaff {
        name: staff_dto.name,
        surname: staff_dto.surname,
        email: staff_dto.email,
        password: hash("123qweasd".to_string(), DEFAULT_COST).unwrap(),
        soft_constraints: sc_id,
        faculty: staff_dto.faculty.parse::<i32>().unwrap(),
        role: role::Role::Staff.to_string()
    };

    let result = staff_repository::create_staff(staff, &connection);
   
    match result {
        Ok(res_staff) => {
            
            let mut lectues_dto = Vec::new();
            
            //add lectures
            let mut lectues = Vec::new();
            
            for lec in staff_dto.lectures {
                let lecture = NewLecturesStaff{
                    lecture_id: lec.parse::<i32>().unwrap(),
                    staff_id: res_staff.id
                };
                lectues_dto.push(lec);
                lectues.push(lecture);
            }
            let res_l = lectures_staff_repository::create_lectures_staff(lectues, &connection);
            if res_l.is_err(){
                return res_l.map(|_|  StaffDTO {id: "-1".to_string(), name: "".to_string(), surname: "".to_string(), email: "".to_string(),
                                                faculty: "-1".to_string(), lectures: Vec::new()})
                            .map_err(|error| error);
            }
  
            Ok(StaffDTO{
            id: res_staff.id.to_string(),
            name: res_staff.name,
            surname: res_staff.surname,
            email: res_staff.email,
            faculty: res_staff.faculty.to_string(),
            lectures: lectues_dto 
        })
        
    },
        Err(e) => Err(e),
    }
}

pub fn get_staff(id: i32, pool: &web::Data<Pool>) -> Result<StaffDTO, Error> {
    info!("{}", format!("   Getting staff {}", id));
  
    let connection = pool.get().expect("Connection from pool");

    let mut lectues_dto = Vec::new();
    let lectures = lectures_staff_repository::get_lectures_staff_by_staff_id(id, &connection);


    match lectures {
        Ok(lectures) => {
            for lecture in lectures {
                lectues_dto.push(lecture.lecture_id.to_string());
            }
        }
        Err(_) => ()
    }
    
    let result = staff_repository::get_staff(id, &connection);

    match result {
        Ok(res) => Ok(StaffDTO{
            id: res.id.to_string(),
            name: res.name,
            surname: res.surname,
            email: res.email,
            faculty: res.faculty.to_string(),
            lectures: lectues_dto 
        }),
        Err(e) => Err(e),
    }
}

pub fn get_all_staff(pool: &web::Data<Pool>) -> Result<Vec<StaffDTO>, Error> {
    info!("   Getting all staff");
  
    let connection = pool.get().expect("Connection from pool");

    let result = staff_repository::get_all_staff(&connection);

    match result {
        Ok(res) => {
        let mut vec_staff = Vec::new();
        
        for staff in res {

            let mut lectures_dto = Vec::new();
            let lectures = lectures_staff_repository::get_lectures_staff_by_staff_id(staff.id, &connection);

            match lectures {
                Ok(lectures) => {
                    for lecture in lectures {
                        lectures_dto.push(lecture.lecture_id.to_string());
                    }
                }
                Err(_) => ()
            }

            let dto = StaffDTO{
                id: staff.id.to_string(),
                name: staff.name,
                surname: staff.surname,
                email: staff.email,
                faculty: staff.faculty.to_string(),
                lectures: lectures_dto 
            };
            vec_staff.push(dto);
        }
        Ok(vec_staff)},
        Err(e) => Err(e),
    }
}

pub fn update_staff(staff_dto: StaffDTO, pool: &web::Data<Pool>) -> Result<StaffDTO, Error> {
    info!("{}", format!("   Update staff {}", staff_dto.id));
  
    let connection = pool.get().expect("Connection from pool");

    let staff_id_int = staff_dto.id.parse::<i32>().unwrap();
    let mut lec_vec = Vec::new();

    for lec in &staff_dto.lectures {
        let lecture = NewLecturesStaff{
            lecture_id: lec.parse::<i32>().unwrap(),
            staff_id: staff_id_int
        };
        lec_vec.push(lecture);
    }

    lectures_staff_repository::update_lectures_staff(lec_vec, staff_id_int, &connection)?;
    let staff_old = staff_repository::get_staff(staff_id_int, &connection).unwrap();

    let staff = Staff {
        id: staff_id_int,
        name: staff_dto.name,
        surname: staff_dto.surname,
        email: staff_dto.email,
        password: hash("123qweasd".to_string(), DEFAULT_COST).unwrap(),
        soft_constraints: staff_old.soft_constraints,
        faculty: staff_dto.faculty.parse::<i32>().unwrap(),
        role: role::Role::Staff.to_string()
    };

    let result = staff_repository::update_staff(staff.id, staff, &connection);

    match result {
        Ok(res) => Ok(StaffDTO{
            id: res.id.to_string(),
            name: res.name,
            surname: res.surname,
            email: res.email,
            faculty: res.faculty.to_string(),
            lectures: staff_dto.lectures 
        }),
        Err(e) => Err(e),
    }
}

pub fn delete_staff(id: i32, pool: &web::Data<Pool>) -> Result<String, ActixError> {
    info!("{}", format!("   Deleting staff {}", id));

    let connection = pool.get().expect("Connection from pool");

    let staff = staff_repository::get_staff(id, &connection).unwrap();

    lectures_staff_repository::delete_lectures_staff(id, &connection).unwrap();
    soft_constraints_repository::delete_soft_constraints(staff.soft_constraints, &connection).unwrap();
    
    return staff_repository::delete_staff(id, &connection)
        .map(|_| "   Staff successfully deleted!".to_string())
        .map_err(|error| response_util::error_response(error));
}

pub fn get_staff_lectures(id: i32, pool: &web::Data<Pool>) -> Result<IdsDTO, Error> {
    info!("   Getting lectures_staff");
  
    let connection = pool.get().expect("Connection from pool");

    let result = lectures_staff_repository::get_lectures_staff_by_staff_id(id, &connection);

    match result {
        Ok(res) => {
        let mut vec_ids = Vec::new();
        
        for lec in res {
            vec_ids.push(lec.lecture_id.to_string());
        }

        let ids_dto = IdsDTO{
            ids: vec_ids,
        };
        Ok(ids_dto)},
        Err(e) => Err(e),
    }
}

pub fn delete_staff_by_faculty(faculty_id: i32, pool: &web::Data<Pool>) -> Result<String, ActixError> {
    info!("{}", format!("   Deleting staff by faculty {}", faculty_id));

    let connection = pool.get().expect("Connection from pool");

    let staff_vec = staff_repository::get_all_staff_by_faculty(faculty_id, &connection).unwrap();

    for staff in staff_vec {
        
        lectures_staff_repository::delete_lectures_staff(staff.id, &connection).unwrap();
        soft_constraints_repository::delete_soft_constraints(staff.soft_constraints, &connection).unwrap();
        
        let res = staff_repository::delete_staff(staff.id, &connection);

        if res.is_err(){
            return Err(response_util::error_response(Error::NotFound))
        }
    }

    Ok( "   Staff successfully deleted!".to_string())
}

