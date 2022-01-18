use crate::{
    schema::shifts::dsl::*,
    models::shift::{Shift, NewShift}
};
use diesel::prelude::*;
use diesel::result::Error;

pub fn create_shift(shift: NewShift, conn: &PgConnection) -> QueryResult<Shift> {
    info!("{}", format!("   Inserting shift"));
    diesel::insert_into(shifts)
        .values(&shift)
        .get_result(conn)
}

pub fn get_shift(shift_id: i32, conn: &PgConnection) -> QueryResult<Shift> {
    info!("   Getting shift by id");
    shifts.find(shift_id).get_result::<Shift>(conn)
}

pub fn update_shift(shift_id: i32, shift: Shift, conn: &PgConnection) -> QueryResult<Shift> {
    info!("{}", format!("   Updating shift {}", &shift_id));
    diesel::update(shifts.find(shift_id))
        .set(&shift)
        .get_result(conn)
}

pub fn get_all_shifts(conn: &PgConnection) -> QueryResult<Vec<Shift>> {
    info!("  Gettinng all shifts");
    shifts.load::<Shift>(conn)
}


pub fn delete_shift(shift_id: i32, conn: &PgConnection) -> QueryResult<usize> {
    info!("   Deleting shift from database");

    let old = shifts.load::<Shift>(conn).unwrap();
   
    diesel::delete(shifts.filter(id.eq(shift_id))).execute(conn)?;
    let new = shifts.load::<Shift>(conn).unwrap();

    if old.len() >= 1&& old.len() - 1 == new.len() {
        Ok(new.len())
    }else{
        Err(Error::NotFound)
    }
}