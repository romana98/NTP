use crate::{
    config::db::Pool,
    models::{algorithm::{Schedule}, schedule::{ScheduleDTO, ScheduleFacultyDTO}, shifts_lecture::{NewShiftsLecture}},
    repository::{schedule_repository, assigned_shifts_repository, shifts_lecture_repository},
    utils::response_util
};
use actix_web::{web, Error as ActixError};
use diesel::result::Error;

pub fn save_schedule(schedule: Schedule, pool: &web::Data<Pool>) -> Result<ScheduleFacultyDTO, Error> {
    info!("{}", format!("   Inserting schdule"));
    
    let connection = pool.get().expect("Connection from pool");

    let result = schedule_repository::create_schedule(&connection);

    match result {
        Ok(res) => {
            let schedule_id = res.id;
            let mut schedule_dto_vec = Vec::new();

            for assigned_shift in schedule {
                let assigned_shifts = assigned_shifts_repository::create_asigned_shift(schedule_id, assigned_shift.staff_id, assigned_shift.staff, &connection).unwrap();


                let mut lectures = Vec::new();
                for lecture in &assigned_shift.lectures {
                    for _ in 0..lecture.number_of_times {
                        lectures.push((*lecture.name).to_owned());
                    }
                }

                let mut shift_lecture_map = Vec::new();
                let mut cnt = 0;
                for (day, vertex_vec) in  &assigned_shift.shifts {
                    for vertex in vertex_vec {
                        shift_lecture_map.push(NewShiftsLecture{
                            assigned_shifts_id: assigned_shifts.id,
                            day: day.to_string(),
                            shift: format!("{}-{}", vertex.start, vertex.end),
                            lecture: (*lectures.get(cnt).unwrap()).to_owned()
                        });
                        
                        schedule_dto_vec.push(ScheduleDTO{
                            staff: (*assigned_shifts.staff).to_owned(),
                            day: day.to_string(),
                            shift: format!("{}-{}", vertex.start, vertex.end),
                            lecture: (*lectures.get(cnt).unwrap()).to_owned()
                        });
                        cnt+=1;
                    }
                }
                shifts_lecture_repository::create_shifts_lectures(shift_lecture_map, &connection).unwrap();
            }
            Ok(ScheduleFacultyDTO{schedule: schedule_dto_vec, schedule_id: schedule_id})
        },
        Err(e) => Err(e),
    }
}

pub fn get_schedule(id: i32, pool: &web::Data<Pool>) -> Result<Vec<ScheduleDTO>, Error> {
    info!("{}", format!("   Getting lecture {}", id));
  
    let connection = pool.get().expect("Connection from pool");

    let assigned_shifts_res = assigned_shifts_repository::get_asigned_shift_by_schedule_id(id, &connection);
    let mut schedule = Vec::new();

    match assigned_shifts_res {
        Ok(assigned_shifts) => {
            for assigned_shift in &assigned_shifts {
                let shifts_lectures = shifts_lecture_repository::get_shifts_lectures_by_assigned_shifts_id(assigned_shift.id, &connection).unwrap();
                for shifts_lecture in shifts_lectures {
                    schedule.push(ScheduleDTO{
                        day: shifts_lecture.day,
                        lecture: shifts_lecture.lecture,
                        shift: shifts_lecture.shift,
                        staff: (*assigned_shift.staff).to_owned()
                    });   
                }
            }
            Ok(schedule)
        },
        Err(e) => Err(e)
    }   
}

pub fn get_schedule_by_staff(id: i32, pool: &web::Data<Pool>) -> Result<Vec<ScheduleDTO>, Error> {
    info!("{}", format!("   Getting lecture {}", id));
  
    let connection = pool.get().expect("Connection from pool");

    let assigned_shifts_res = assigned_shifts_repository::get_asigned_shift_by_staff_id(id, &connection);
    let mut schedule = Vec::new();

    match assigned_shifts_res {
        Ok(assigned_shifts) => {
            for assigned_shift in &assigned_shifts {
                let shifts_lectures = shifts_lecture_repository::get_shifts_lectures_by_assigned_shifts_id(assigned_shift.id, &connection).unwrap();
                for shifts_lecture in shifts_lectures {
                    schedule.push(ScheduleDTO{
                        day: shifts_lecture.day,
                        lecture: shifts_lecture.lecture,
                        shift: shifts_lecture.shift,
                        staff: (*assigned_shift.staff).to_owned()
                    });   
                }
            }
            Ok(schedule)
        },
        Err(e) => Err(e)
    }   
}

pub fn delete_schedule(id: i32, pool: &web::Data<Pool>) -> Result<String, ActixError> {
    info!("{}", format!("   Deleting schedule {}", id));

    let connection = pool.get().expect("Connection from pool");

    let assigned_shifts = assigned_shifts_repository::get_asigned_shift_by_schedule_id(id, &connection).unwrap();

    for assigned_shift in assigned_shifts {
        shifts_lecture_repository::delete_shifts_lectures(assigned_shift.id, &connection).unwrap();
    }

    assigned_shifts_repository::delete_asigned_shift(id, &connection).unwrap();
    
    return schedule_repository::delete_schedule(id, &connection)
        .map(|_| "   Schedule successfully deleted!".to_string())
        .map_err(|error| response_util::error_response(error));
}