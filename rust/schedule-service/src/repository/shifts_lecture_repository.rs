use crate::{
    schema::shifts_lectures::dsl::*,
    models::shifts_lecture::{NewShiftsLecture, ShiftsLecture},
    //repository::prefers_repository
};
use diesel::prelude::*;
//use diesel::result::Error;


pub fn create_shifts_lectures(shifts_lectures_vec: Vec<NewShiftsLecture>, conn: &PgConnection) -> QueryResult<Vec<ShiftsLecture>> {
    info!("{}", format!("   Inserting shifts_lectures"));

    diesel::insert_into(shifts_lectures)
        .values(&shifts_lectures_vec)
        .get_results(conn)
}