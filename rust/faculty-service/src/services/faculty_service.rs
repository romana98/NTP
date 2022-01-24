use crate::{
    config::db::Pool,
    models::{faculty::{FacultyDTO, Faculty, NewFaculty}, lectures_faculty::{NewLecturesFaculty}, 
            shifts_faculty::{NewShiftsFaculty}, schedule::{FacultySchedule, IdsDTO}, hard_constraints::{HardConstraintDTO},
            shift::{ShiftDTO}},
    utils::{response_util},
    repository::{faculty_repository, staff_faculty_repository, shifts_faculty_repository, lectures_faculty_repository, shift_repository, hard_constraints_repository},
};
use actix_web::{web, Error as ActixError};
use diesel::result::Error;

pub fn create_faculty(faculty_dto: FacultyDTO, pool: &web::Data<Pool>) -> Result<FacultyDTO, Error> {
    info!("{}", format!("   Inserting faculty"));

    let schedule = match &faculty_dto.schedule as &str {
                    "" =>None::<i32>,
                    _ => Some(faculty_dto.schedule.parse::<i32>().unwrap())
                };
                
    let faculty = NewFaculty {
        name: faculty_dto.name,
        hard_constraint_id: faculty_dto.hard_constraint.parse::<i32>().unwrap(),
        schedule_id: schedule
    };
    let connection = pool.get().expect("Connection from pool");

    let result = faculty_repository::create_faculty(faculty, &connection);


    match result {
        Ok(res) => {
            //add lectures
            let mut vec_lectures = Vec::new();
            let mut vec_lectures_dto = Vec::new();
            for lecture in faculty_dto.lectures {
                let lec = NewLecturesFaculty{
                    faculty_id: res.id,
                    lecture_id: lecture.parse::<i32>().unwrap()
                };
                vec_lectures.push(lec);
                vec_lectures_dto.push(lecture);
            }
            let result_lecture = lectures_faculty_repository::create_lectures_faculty(vec_lectures, &connection);

            if result_lecture.is_err(){
                return result_lecture.map(|_|  FacultyDTO {id: "-1".to_string(), name: "".to_string(), hard_constraint: "".to_string(), schedule: "".to_string(),
                                        staff :Vec::new(), shifts:Vec::new(), lectures: Vec::new()})
                            .map_err(|error| error);
            }
            
            //add shifts
            let mut vec_shifts = Vec::new();
            let mut vec_shifts_dto = Vec::new();
            for shift in faculty_dto.shifts {
                let sh = NewShiftsFaculty{
                    faculty_id: res.id,
                    shift_id: shift.parse::<i32>().unwrap()
                };
                vec_shifts.push(sh);
                vec_shifts_dto.push(shift);
            }
            let result_shifts = shifts_faculty_repository::create_shifts_faculty(vec_shifts, &connection);

            if result_shifts.is_err(){
                return result_shifts.map(|_|  FacultyDTO {id: "-1".to_string(), name: "".to_string(), hard_constraint: "".to_string(), schedule: "".to_string(),
                                        staff :Vec::new(), shifts:Vec::new(), lectures: Vec::new()})
                            .map_err(|error| error);
            }

            if res.schedule_id.is_some(){
                Ok(FacultyDTO{
                    id: res.id.to_string(),
                    name: res.name,
                    hard_constraint: res.hard_constraint_id.to_string(),
                    schedule: res.schedule_id.unwrap().to_string(),
                    lectures: vec_lectures_dto,
                    staff: Vec::new(),
                    shifts: vec_shifts_dto}
                )
            }else {
                Ok(FacultyDTO{
                    id: res.id.to_string(),
                    name: res.name,
                    hard_constraint: res.hard_constraint_id.to_string(),
                    schedule: "".to_string(),
                    lectures: vec_lectures_dto,
                    staff:  Vec::new(),
                    shifts: vec_shifts_dto}
                )
            } 
        },
        Err(e) => Err(e)
    }
}

pub fn get_faculty(id: i32, pool: &web::Data<Pool>) -> Result<FacultyDTO, Error> {
    info!("{}", format!("   Getting faculty {}", id));
  
    let connection = pool.get().expect("Connection from pool");

    let mut lectures_dto = Vec::new();
    let mut staff_dto = Vec::new();
    let mut shifts_dto = Vec::new();

    let lectures = lectures_faculty_repository::get_lectures_faculty_by_faculty_id(id, &connection);

    match lectures {
        Ok(lectures) => {
            for lecture in lectures {
                lectures_dto.push(lecture.lecture_id.to_string());
            }
        }
        Err(_) => ()
    }

    let staff = staff_faculty_repository::get_staff_faculty_by_faculty_id(id, &connection);

    match staff {
        Ok(staff) => {
            for st in staff {
                staff_dto.push(st.staff_id.to_string());
            }
        }
        Err(_) => ()
    }

    let shifts = shifts_faculty_repository::get_shifts_faculty_by_faculty_id(id, &connection);

    match shifts {
        Ok(shifts) => {
            for sh in shifts {
                shifts_dto.push(sh.shift_id.to_string());
            }
        }
        Err(_) => ()
    }
    let result = faculty_repository::get_faculty(id, &connection);

    match result {
        Ok(res) => {
    
            if res.schedule_id.is_some(){
                Ok(FacultyDTO{
                    id: res.id.to_string(),
                    name: res.name,
                    hard_constraint: res.hard_constraint_id.to_string(),
                    schedule: res.schedule_id.unwrap().to_string(),
                    lectures: lectures_dto,
                    staff: staff_dto,
                    shifts: shifts_dto}
                )
            }else {
                Ok(FacultyDTO{
                    id: res.id.to_string(),
                    name: res.name,
                    hard_constraint: res.hard_constraint_id.to_string(),
                    schedule: "".to_string(),
                    lectures: lectures_dto,
                    staff: staff_dto,
                    shifts: shifts_dto}
                )
            } 
        },
        Err(e) => Err(e),
    }
}

pub fn delete_faculty(id: i32, pool: &web::Data<Pool>) -> Result<String, ActixError> {
    info!("{}", format!("   Deleting faculty {}", id));

    let connection = pool.get().expect("Connection from pool");

    lectures_faculty_repository::delete_lectures_faculty(id, &connection).unwrap();
    shifts_faculty_repository::delete_shifts_faculty(id, &connection).unwrap();
    staff_faculty_repository::delete_staff_faculty(id, &connection).unwrap();
    
    return faculty_repository::delete_faculty(id, &connection)
        .map(|_| "   Faculty successfully deleted!".to_string())
        .map_err(|error| response_util::error_response(error));
}

pub fn update_faculty(faculty_dto: FacultyDTO, pool: &web::Data<Pool>) -> Result<FacultyDTO, Error> {
    info!("{}", format!("   Update faculty {}", faculty_dto.id));
  
    let connection = pool.get().expect("Connection from pool");

    let faculty_id_int = faculty_dto.id.parse::<i32>().unwrap();

    let mut lec_vec = Vec::new();

    for lec in &faculty_dto.lectures {
        let lecture = NewLecturesFaculty{
            lecture_id: lec.parse::<i32>().unwrap(),
            faculty_id: faculty_id_int
        };
        lec_vec.push(lecture);
    }
    lectures_faculty_repository::update_lectures_faculty(lec_vec, faculty_id_int, &connection)?;

    let mut shifts_vec = Vec::new();

    for shift in &faculty_dto.shifts {
        let sh = NewShiftsFaculty{
            shift_id: shift.parse::<i32>().unwrap(),
            faculty_id: faculty_id_int
        };
        shifts_vec.push(sh);
    }
    shifts_faculty_repository::update_shifts_faculty(shifts_vec, faculty_id_int, &connection)?;

    let schedule = match &faculty_dto.schedule as &str {
        "" =>None::<i32>,
        _ => Some(faculty_dto.schedule.parse::<i32>().unwrap())
    };
    let faculty = Faculty {
        id: faculty_dto.id.parse::<i32>().unwrap(),
        name: faculty_dto.name,
        hard_constraint_id: faculty_dto.hard_constraint.parse::<i32>().unwrap(),
        schedule_id: schedule
    };

    let staff_result = staff_faculty_repository::get_staff_faculty_by_faculty_id(faculty_id_int, &connection);

    let mut staff_dto = Vec::new();
    if let Ok(staff) = staff_result.as_ref() {
        for st in staff {
            staff_dto.push(st.staff_id.to_string());
        }
    }

    let result = faculty_repository::update_faculty(faculty_dto.id.parse::<i32>().unwrap(), faculty, &connection);

    match result {
        Ok(res) => {
            if res.schedule_id.is_some(){
                Ok(FacultyDTO{
                    id: res.id.to_string(),
                    name: res.name,
                    hard_constraint: res.hard_constraint_id.to_string(),
                    schedule: res.schedule_id.unwrap().to_string(),
                    lectures: faculty_dto.lectures,
                    staff: staff_dto,
                    shifts: faculty_dto.shifts
                })
            }else {
                Ok(FacultyDTO{
                    id: res.id.to_string(),
                    name: res.name,
                    hard_constraint: res.hard_constraint_id.to_string(),
                    schedule: "".to_string(),
                    lectures: faculty_dto.lectures,
                    staff: faculty_dto.staff,
                    shifts: faculty_dto.shifts
                })
            } 
        },
        Err(e) => Err(e),
    }
}

pub fn get_all_faculties(pool: &web::Data<Pool>) -> Result<Vec<FacultyDTO>, Error> {
    info!("   Get all faculties");
  
    let connection = pool.get().expect("Connection from pool");

    let result = faculty_repository::get_all_faculties(&connection);

    match result {
        Ok(res) => {
        let mut vec_faculties = Vec::new();
        
        for faculty in res {

            let mut lectures_dto = Vec::new();
            let lectures = lectures_faculty_repository::get_lectures_faculty_by_faculty_id(faculty.id, &connection);

            match lectures {
                Ok(lectures) => {
                    for lecture in lectures {
                        lectures_dto.push(lecture.lecture_id.to_string());
                    }
                }
                Err(_) => ()
            }

            let mut staff_dto = Vec::new();
            let staff = staff_faculty_repository::get_staff_faculty_by_faculty_id(faculty.id, &connection);

            match staff {
                Ok(staff) => {
                    for st in staff {
                        staff_dto.push(st.staff_id.to_string());
                    }
                }
                Err(_) => ()
            }

            let mut shifts_dto = Vec::new();
            let shifts = shifts_faculty_repository::get_shifts_faculty_by_faculty_id(faculty.id, &connection);

            match shifts {
                Ok(shifts) => {
                    for st in shifts {
                        shifts_dto.push(st.shift_id.to_string());
                    }
                }
                Err(_) => ()
            }

            if faculty.schedule_id.is_some(){
                let dto = FacultyDTO{
                    id: faculty.id.to_string(),
                    name: faculty.name,
                    hard_constraint: faculty.hard_constraint_id.to_string(),
                    schedule: faculty.schedule_id.unwrap().to_string(),
                    lectures: lectures_dto,
                    staff: staff_dto,
                    shifts: shifts_dto};
                vec_faculties.push(dto);  
            }else {
                let dto = FacultyDTO{
                    id: faculty.id.to_string(),
                    name: faculty.name,
                    hard_constraint: faculty.hard_constraint_id.to_string(),
                    schedule: "".to_string(),
                    lectures: lectures_dto,
                    staff: staff_dto,
                    shifts: shifts_dto};
                vec_faculties.push(dto);  
            }    
        }
        Ok(vec_faculties)},
        Err(e) => Err(e),
    }
}

pub fn get_faculty_for_schedule(id: i32, pool: &web::Data<Pool>) -> Result<FacultySchedule, Error> {
    info!("{}", format!("   Getting faculty {}", id));
  
    let connection = pool.get().expect("Connection from pool");

    let mut lectures_dto = Vec::new();
    let mut staff_dto = Vec::new();
    let mut shifts_dto = Vec::new();
    let mut shifts_ids = Vec::new();

    let lectures = lectures_faculty_repository::get_lectures_faculty_by_faculty_id(id, &connection);

    match lectures {
        Ok(lectures) => {
            for lecture in lectures {
                lectures_dto.push(lecture.lecture_id);
            }
        }
        Err(_) => ()
    }

    let staff = staff_faculty_repository::get_staff_faculty_by_faculty_id(id, &connection);

    match staff {
        Ok(staff) => {
            for st in staff {
                staff_dto.push(st.staff_id);
            }
        }
        Err(_) => ()
    }

    let shifts_faculty = shifts_faculty_repository::get_shifts_faculty_by_faculty_id(id, &connection);

    match shifts_faculty {
        Ok(shifts) => {
            for sh in shifts {
                shifts_ids.push(sh.shift_id);
            }
        }
        Err(_) => ()
    }

    let shifts = shift_repository::get_all_shifts_by_ids(shifts_ids, &connection);

    if shifts.is_err(){
        return shifts.map(|_|  FacultySchedule {id: -1, schedule_id: -1, name: "".to_string(), 
                                            hard_constraint: HardConstraintDTO{id:"".to_string(), daily_max: -1, max_per_shift: -1, weekly_max: -1, weekly_min: -1},
                                            shifts: Vec::new(), staff: Vec::new(), lectures: Vec::new()})
                 .map_err(|error| error);
     }else if let Ok(shifts_vec) = shifts {
        for shift in shifts_vec {
            shifts_dto.push(ShiftDTO{
                id: shift.id.to_string(),
                day: shift.day,
                start: shift.start_shift,
                end: shift.end_shift
            });
        }
     }

    let result = faculty_repository::get_faculty(id, &connection);

    match result {
        Ok(res) => {
            let mut hc = HardConstraintDTO{id:"".to_string(), daily_max: -1, max_per_shift: -1, weekly_max: -1, weekly_min: -1};
            let hc_result = hard_constraints_repository::get_hard_constraint(res.hard_constraint_id, &connection);

            if hc_result.is_err(){
                return hc_result.map(|_|  FacultySchedule {id: -1, schedule_id: -1, name: "".to_string(), 
                                                    hard_constraint: HardConstraintDTO{id:"".to_string(), daily_max: -1, max_per_shift: -1, weekly_max: -1, weekly_min: -1},
                                                    shifts: Vec::new(), staff: Vec::new(), lectures: Vec::new()})
                         .map_err(|error| error);
            }else if let Ok(hard_const) = hc_result.as_ref() {
                
                   hc = HardConstraintDTO{
                        id: hard_const.id.to_string(),
                        daily_max: hard_const.daily_max,
                        weekly_min: hard_const.weekly_min,
                        weekly_max: hard_const.weekly_max,
                        max_per_shift: hard_const.max_per_shift
                    };
             }

             if res.schedule_id.is_some(){
                Ok(FacultySchedule{
                    id: res.id,
                    schedule_id: res.schedule_id.unwrap(),
                    name: res.name,
                    hard_constraint: hc,
                    lectures: lectures_dto,
                    staff: staff_dto,
                    shifts: shifts_dto}
                )
            }else {
                Ok(FacultySchedule{
                    id: res.id,
                    schedule_id: 0,
                    name: res.name,
                    hard_constraint: hc,
                    lectures: lectures_dto,
                    staff: staff_dto,
                    shifts: shifts_dto}
                )
            }      
        },
        Err(e) => Err(e),
    }
}

pub fn update_faculty_add_schedule(ids_dto: IdsDTO, pool: &web::Data<Pool>) -> Result<bool, Error> {
    info!("{}", format!("   Update faculty - add schedule {}", ids_dto.faculty_id));
  
    let connection = pool.get().expect("Connection from pool");

    let result = faculty_repository::get_faculty(ids_dto.faculty_id, &connection).unwrap();

    let faculty = Faculty {
        id: result.id,
        name: result.name,
        hard_constraint_id: result.hard_constraint_id,
        schedule_id: Some(ids_dto.schedule_id)
    };

    let result = faculty_repository::update_faculty(ids_dto.faculty_id, faculty, &connection);

    match result {
        Ok(_) => {
            Ok(true)
        },
        Err(e) => Err(e),
    }
}
