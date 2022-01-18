use crate::{
    schema::lectures_staffs::dsl::*,
    models::lectures_staffs::{LecturesStaff, NewLecturesStaff}
};
use diesel::prelude::*;
use diesel::result::Error;


pub fn create_lectures_staff(lec_staff: Vec<NewLecturesStaff>, conn: &PgConnection) -> QueryResult<Vec<LecturesStaff>> {
    info!("{}", format!("   Inserting lectures_staff"));
    diesel::insert_into(lectures_staffs)
        .values(&lec_staff)
        .get_results(conn)
}

pub fn get_lectures_staff_by_staff_id(staff_id_id: i32, conn: &PgConnection) -> QueryResult<Vec<LecturesStaff>> {
    info!("{}", format!("   Getting lectures_staff by staff_id"));
    lectures_staffs.filter(staff_id.eq(staff_id_id)).load::<LecturesStaff>(conn)
}

pub fn update_lectures_staff(lec_staff: Vec<NewLecturesStaff>, staff_id_id: i32, conn: &PgConnection) -> QueryResult<Vec<LecturesStaff>> {
    info!("{}", format!("   Update lectures_staff by staff_id"));
    
    delete_lectures_staff(staff_id_id, conn)?;
    create_lectures_staff(lec_staff, conn)
}

pub fn delete_lectures_staff(staff_id_id: i32, conn: &PgConnection) -> QueryResult<usize> {
    info!("   Deleting lectures_staff from database");

    let old = lectures_staffs.load::<LecturesStaff>(conn).unwrap();
    let num = lectures_staffs.filter(staff_id.eq(staff_id_id)).load::<LecturesStaff>(conn).unwrap().len();
   
    diesel::delete(lectures_staffs.filter(staff_id.eq(staff_id_id))).execute(conn)?;
    let new = lectures_staffs.load::<LecturesStaff>(conn).unwrap();

    if old.len() >= num && old.len() - num == new.len() {
        Ok(new.len())
    }else{
        Err(Error::NotFound)
    }
}

pub fn delete_lectures_staff_by_lecture(lecture_id_id: i32, conn: &PgConnection) -> QueryResult<usize> {
    info!("   Deleting lectures_staff from database");

    let old = lectures_staffs.load::<LecturesStaff>(conn).unwrap();
    let num = lectures_staffs.filter(lecture_id.eq(lecture_id_id)).load::<LecturesStaff>(conn).unwrap().len();
   
    diesel::delete(lectures_staffs.filter(lecture_id.eq(lecture_id_id))).execute(conn)?;
    let new = lectures_staffs.load::<LecturesStaff>(conn).unwrap();

    if old.len() >= num && old.len() - num == new.len() {
        Ok(new.len())
    }else{
        Err(Error::NotFound)
    }
}