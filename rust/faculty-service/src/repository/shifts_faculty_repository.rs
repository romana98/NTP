use crate::{
    schema::shifts_faculties::dsl::*,
    models::shifts_faculty::{ShiftsFaculty, NewShiftsFaculty}
};
use diesel::prelude::*;
use diesel::result::Error;


pub fn create_shifts_faculty(shifts_faculty: Vec<NewShiftsFaculty>, conn: &PgConnection) -> QueryResult<Vec<ShiftsFaculty>> {
    info!("{}", format!("   Inserting shifts_faculty"));
    diesel::insert_into(shifts_faculties)
        .values(&shifts_faculty)
        .get_results(conn)
}

pub fn get_shifts_faculty_by_faculty_id(faculty_id_id: i32, conn: &PgConnection) -> QueryResult<Vec<ShiftsFaculty>> {
    info!("{}", format!("   Getting shifts_faculty by faculty_id"));
    shifts_faculties.filter(faculty_id.eq(faculty_id_id)).load::<ShiftsFaculty>(conn)
}

pub fn update_shifts_faculty(shifts_faculty: Vec<NewShiftsFaculty>, faculty_id_id: i32, conn: &PgConnection) -> QueryResult<Vec<ShiftsFaculty>> {
    info!("{}", format!("   Update shifts_faculty by staff_id"));
    
    delete_shifts_faculty(faculty_id_id, conn)?;
    create_shifts_faculty(shifts_faculty, conn)
}

pub fn delete_shifts_faculty(faculty_id_id: i32, conn: &PgConnection) -> QueryResult<usize> {
    info!("   Deleting shifts_faculty from database");

    let old = shifts_faculties.load::<ShiftsFaculty>(conn).unwrap();
    let num = shifts_faculties.filter(faculty_id.eq(faculty_id_id)).load::<ShiftsFaculty>(conn).unwrap().len();
   
    diesel::delete(shifts_faculties.filter(faculty_id.eq(faculty_id_id))).execute(conn)?;
    let new = shifts_faculties.load::<ShiftsFaculty>(conn).unwrap();

    if old.len() >= num && old.len() - num == new.len() {
        Ok(new.len())
    }else{
        Err(Error::NotFound)
    }
}

pub fn delete_shifts_faculty_by_shift_id(shift_id_id: i32, conn: &PgConnection) -> QueryResult<usize> {
    info!("   Deleting shifts_faculty from database");

    let old = shifts_faculties.load::<ShiftsFaculty>(conn).unwrap();
    let num = shifts_faculties.filter(shift_id.eq(shift_id_id)).load::<ShiftsFaculty>(conn).unwrap().len();
   
    diesel::delete(shifts_faculties.filter(shift_id.eq(shift_id_id))).execute(conn)?;
    let new = shifts_faculties.load::<ShiftsFaculty>(conn).unwrap();

    if old.len() >= num && old.len() - num == new.len() {
        Ok(new.len())
    }else{
        Err(Error::NotFound)
    }
}