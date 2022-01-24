use crate::{
    schema::shifts_lectures::dsl::*,
    models::shifts_lecture::{NewShiftsLecture, ShiftsLecture}
};
use diesel::prelude::*;
use diesel::result::Error;


pub fn create_shifts_lectures(shifts_lectures_vec: Vec<NewShiftsLecture>, conn: &PgConnection) -> QueryResult<Vec<ShiftsLecture>> {
    info!("{}", format!("   Inserting shifts_lectures"));

    diesel::insert_into(shifts_lectures)
        .values(&shifts_lectures_vec)
        .get_results(conn)
}

pub fn get_shifts_lectures_by_assigned_shifts_id(asigned_shift_id_id: i32, conn: &PgConnection) -> QueryResult<Vec<ShiftsLecture>> {
    info!("{}", format!("   Getting shifts_lectures by asigned_shift_id"));
    shifts_lectures.filter(assigned_shifts_id.eq(asigned_shift_id_id)).load::<ShiftsLecture>(conn)
}

pub fn delete_shifts_lectures(assigned_shifts_id_id: i32, conn: &PgConnection) -> QueryResult<usize> {
    info!("   Deleting shifts_lectures from database");

    let old = shifts_lectures.load::<ShiftsLecture>(conn).unwrap();
    let num = shifts_lectures.filter(assigned_shifts_id.eq(assigned_shifts_id_id)).load::<ShiftsLecture>(conn).unwrap().len();
   
    diesel::delete(shifts_lectures.filter(assigned_shifts_id.eq(assigned_shifts_id_id))).execute(conn)?;
    let new = shifts_lectures.load::<ShiftsLecture>(conn).unwrap();

    if old.len() >= num && old.len() - num == new.len() {
        Ok(new.len())
    }else{
        Err(Error::NotFound)
    }
}