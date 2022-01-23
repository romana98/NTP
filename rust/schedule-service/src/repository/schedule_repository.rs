use crate::{
    schema::schedules::dsl::*,
    models::schedule::{NewSchedule, Schedule},
    //repository::prefers_repository
};
use diesel::prelude::*;
//use diesel::result::Error;


pub fn create_schedule(conn: &PgConnection) -> QueryResult<Schedule> {
    info!("{}", format!("   Inserting schedule"));
    
    let sc = NewSchedule{id: None};

    diesel::insert_into(schedules)
        .values(&sc)
        .get_result(conn)
}