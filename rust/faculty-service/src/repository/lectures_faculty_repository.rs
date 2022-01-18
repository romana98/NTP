use crate::{
    schema::lectures_faculties::dsl::*,
    models::lectures_faculty::{LecturesFaculty, NewLecturesFaculty}
};
use diesel::prelude::*;
use diesel::result::Error;


pub fn create_lectures_faculty(lec_faculty: Vec<NewLecturesFaculty>, conn: &PgConnection) -> QueryResult<Vec<LecturesFaculty>> {
    info!("{}", format!("   Inserting lectures_faculty"));
    diesel::insert_into(lectures_faculties)
        .values(&lec_faculty)
        .get_results(conn)
}

pub fn get_lectures_faculty_by_faculty_id(faculty_id_id: i32, conn: &PgConnection) -> QueryResult<Vec<LecturesFaculty>> {
    info!("{}", format!("   Getting lectures_faculty by faculty_id"));
    lectures_faculties.filter(faculty_id.eq(faculty_id_id)).load::<LecturesFaculty>(conn)
}

pub fn update_lectures_faculty(lec_faculty: Vec<NewLecturesFaculty>, faculty_id_id: i32, conn: &PgConnection) -> QueryResult<Vec<LecturesFaculty>> {
    info!("{}", format!("   Update lectures_faculty by staff_id"));
    
    delete_lectures_faculty(faculty_id_id, conn)?;
    create_lectures_faculty(lec_faculty, conn)
}

pub fn delete_lectures_faculty(faculty_id_id: i32, conn: &PgConnection) -> QueryResult<usize> {
    info!("   Deleting lectures_faculty from database");

    let old = lectures_faculties.load::<LecturesFaculty>(conn).unwrap();
    let num = lectures_faculties.filter(faculty_id.eq(faculty_id_id)).load::<LecturesFaculty>(conn).unwrap().len();
   
    diesel::delete(lectures_faculties.filter(faculty_id.eq(faculty_id_id))).execute(conn)?;
    let new = lectures_faculties.load::<LecturesFaculty>(conn).unwrap();

    if old.len() >= num && old.len() - num == new.len() {
        Ok(new.len())
    }else{
        Err(Error::NotFound)
    }
}

pub fn delete_lectures_faculty_by_lecture(lecture_id_id: i32, conn: &PgConnection) -> QueryResult<usize> {
    info!("   Deleting lectures_faculty from database");

    let old = lectures_faculties.load::<LecturesFaculty>(conn).unwrap();
    let num = lectures_faculties.filter(lecture_id.eq(lecture_id_id)).load::<LecturesFaculty>(conn).unwrap().len();
   
    diesel::delete(lectures_faculties.filter(lecture_id.eq(lecture_id_id))).execute(conn)?;
    let new = lectures_faculties.load::<LecturesFaculty>(conn).unwrap();

    if old.len() >= num && old.len() - num == new.len() {
        Ok(new.len())
    }else{
        Err(Error::NotFound)
    }
}