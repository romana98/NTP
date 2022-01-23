use crate::{
    config::db::Pool,
    models::{algorithm::{Schedule}, schedule::{ScheduleDTO}, shifts_lecture::{NewShiftsLecture}},
    repository::{schedule_repository, assigned_shifts_repository, shifts_lecture_repository},
};
use actix_web::{web, /*Error as ActixError*/};
use diesel::result::Error;

pub fn save_schedule(schedule: Schedule, pool: &web::Data<Pool>) -> Result<Vec<ScheduleDTO>, Error> {
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
                            shift: format!("{}-{}", vertex.end, vertex.start),
                            lecture: (*lectures.get(cnt).unwrap()).to_owned()
                        });
                        
                        schedule_dto_vec.push(ScheduleDTO{
                            staff: (*assigned_shifts.staff).to_owned(),
                            day: day.to_string(),
                            shift: format!("{}-{}", vertex.end, vertex.start),
                            lecture: (*lectures.get(cnt).unwrap()).to_owned()
                        });
                        cnt+=1;
                    }
                }
                shifts_lecture_repository::create_shifts_lectures(shift_lecture_map, &connection).unwrap();
            }
            Ok(schedule_dto_vec)},
        Err(e) => Err(e),
    }
}
