use crate::{
    schema::schedules::dsl::*,
    models::schedule::{NewSchedule, Schedule}
};
use diesel::prelude::*;
use diesel::result::Error;


pub fn create_schedule(conn: &PgConnection) -> QueryResult<Schedule> {
    info!("{}", format!("   Inserting schedule"));
    
    let sc = NewSchedule{id: None};

    diesel::insert_into(schedules)
        .values(&sc)
        .get_result(conn)
}

pub fn delete_schedule(schedule_id: i32, conn: &PgConnection) -> QueryResult<usize> {
    info!("   Deleting schedule from database");

    let old = schedules.load::<Schedule>(conn).unwrap();
   
    diesel::delete(schedules.filter(id.eq(schedule_id))).execute(conn)?;
    let new = schedules.load::<Schedule>(conn).unwrap();

    if old.len() >= 1 && old.len() - 1 == new.len() {
        Ok(new.len())
    }else{
        Err(Error::NotFound)
    }
}