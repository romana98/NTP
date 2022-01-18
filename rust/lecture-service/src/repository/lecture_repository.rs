use crate::{
    schema::lectures::dsl::*,
    models::lecture::{Lecture, NewLecture}
};
use diesel::prelude::*;
use diesel::result::Error;

pub fn create_lecture(lecture: NewLecture, conn: &PgConnection) -> QueryResult<Lecture> {
    info!("{}", format!("   Inserting lecture {}", &lecture.name));
    diesel::insert_into(lectures)
        .values(&lecture)
        .get_result(conn)
}

pub fn get_lecture(lecture_id: i32, conn: &PgConnection) -> QueryResult<Lecture> {
    info!("   Getting lecture by id");
    lectures.find(lecture_id).get_result::<Lecture>(conn)
}

pub fn update_lecture(lecture_id: i32,lecture: Lecture, conn: &PgConnection) -> QueryResult<Lecture> {
    info!("{}", format!("   Updating lecture {}", &lecture.name));
    diesel::update(lectures.find(lecture_id))
        .set(&lecture)
        .get_result(conn)
}

pub fn get_all_lectures(conn: &PgConnection) -> QueryResult<Vec<Lecture>> {
    info!("  Gettinng all lectures");
    lectures.load::<Lecture>(conn)
}

pub fn get_all_lectures_by_ids(ids: Vec<i32>, conn: &PgConnection) -> QueryResult<Vec<Lecture>> {
    info!("  Gettinng all lectures by ids");
    lectures.filter(id.eq_any(ids)).load::<Lecture>(conn)
}

pub fn delete_lecture(lecture_id: i32, conn: &PgConnection) -> QueryResult<usize> {
    info!("   Deleting lecture from database");

    let old = lectures.load::<Lecture>(conn).unwrap();
   
    diesel::delete(lectures.filter(id.eq(lecture_id))).execute(conn)?;
    let new = lectures.load::<Lecture>(conn).unwrap();

    if old.len() >= 1 && old.len() - 1 == new.len() {
        Ok(new.len())
    }else{
        Err(Error::NotFound)
    }
}