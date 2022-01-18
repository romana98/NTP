use crate::{
    schema::staffs_faculties::dsl::*,
    models::staff_faculty::{StaffFaculty, NewStaffFaculty}
};
use diesel::prelude::*;
use diesel::result::Error;

pub fn create_staff_faculty(staff_faculty: NewStaffFaculty, conn: &PgConnection) -> QueryResult<StaffFaculty> {
    info!("{}", format!("   Inserting staff_faculty"));
    diesel::insert_into(staffs_faculties)
        .values(&staff_faculty)
        .get_result(conn)
}

pub fn get_staff_faculty_by_faculty_id(faculty_id_id: i32, conn: &PgConnection) -> QueryResult<Vec<StaffFaculty>> {
    info!("{}", format!("   Getting staff_faculty by faculty_id"));
    staffs_faculties.filter(faculty_id.eq(faculty_id_id)).load::<StaffFaculty>(conn)
}

pub fn update_staff_faculty(staff_faculty: NewStaffFaculty, staff_id_id: i32, conn: &PgConnection) -> QueryResult<StaffFaculty> {
    info!("{}", format!("   Update staff_faculty by staff_id"));
    
    delete_staff_faculty_by_staff(staff_id_id, conn)?;
    create_staff_faculty(staff_faculty, conn)
}

pub fn delete_staff_faculty(faculty_id_id: i32, conn: &PgConnection) -> QueryResult<usize> {
    info!("   Deleting staff_faculty from database");

    let old = staffs_faculties.load::<StaffFaculty>(conn).unwrap();
    let num = staffs_faculties.filter(faculty_id.eq(faculty_id_id)).load::<StaffFaculty>(conn).unwrap().len();
   
    diesel::delete(staffs_faculties.filter(faculty_id.eq(faculty_id_id))).execute(conn)?;
    let new = staffs_faculties.load::<StaffFaculty>(conn).unwrap();

    if old.len() >= num && old.len() - num == new.len() {
        Ok(new.len())
    }else{
        Err(Error::NotFound)
    }
}

pub fn delete_staff_faculty_by_staff(staff_id_id: i32, conn: &PgConnection) -> QueryResult<usize> {
    info!("   Deleting staff_faculty from database");

    let old = staffs_faculties.load::<StaffFaculty>(conn).unwrap();
    let num = staffs_faculties.filter(staff_id.eq(staff_id_id)).load::<StaffFaculty>(conn).unwrap().len();
   
    diesel::delete(staffs_faculties.filter(staff_id.eq(staff_id_id))).execute(conn)?;
    let new = staffs_faculties.load::<StaffFaculty>(conn).unwrap();

    if old.len() >= num && old.len() - num == new.len() {
        Ok(new.len())
    }else{
        Err(Error::NotFound)
    }
}