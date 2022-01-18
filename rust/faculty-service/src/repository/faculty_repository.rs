use crate::{
    schema::faculties::dsl::*,
    models::faculty::{NewFaculty, Faculty}
};
use diesel::prelude::*;
use diesel::result::Error;

pub fn create_faculty(faculty: NewFaculty, conn: &PgConnection) -> QueryResult<Faculty> {
    info!("{}", format!("   Inserting faculty"));
    diesel::insert_into(faculties)
        .values(&faculty)
        .get_result(conn)
}

pub fn get_faculty(faculty_id: i32, conn: &PgConnection) -> QueryResult<Faculty> {
    info!("   Getting faculty by id");
    faculties.find(faculty_id).get_result::<Faculty>(conn)
}

pub fn update_faculty(faculty_id: i32, faculty: Faculty, conn: &PgConnection) -> QueryResult<Faculty> {
    info!("{}", format!("   Updating faculty {}", &faculty_id));
    diesel::update(faculties.find(faculty_id))
        .set(&faculty)
        .get_result(conn)
}

pub fn get_all_faculties(conn: &PgConnection) -> QueryResult<Vec<Faculty>> {
    info!("  Gettinng all faculties");
    faculties.load::<Faculty>(conn)
}

pub fn delete_faculty(faculty_id: i32, conn: &PgConnection) -> QueryResult<usize> {
    info!("   Deleting faculty from database");

    let old = faculties.load::<Faculty>(conn).unwrap();
   
    diesel::delete(faculties.filter(id.eq(faculty_id))).execute(conn)?;
    let new = faculties.load::<Faculty>(conn).unwrap();

    if old.len() >= 1 && old.len() - 1 == new.len() {
        Ok(new.len())
    }else{
        Err(Error::NotFound)
    }
}