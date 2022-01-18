use crate::{
    schema::staffs::dsl::*,
    models::staff::{Staff, NewStaff}
};
use diesel::prelude::*;
use diesel::result::Error;

pub fn create_staff(staff: NewStaff, conn: &PgConnection) -> QueryResult<Staff> {
    info!("{}", format!("   Inserting staff {} {}", &staff.name, &staff.surname));
    diesel::insert_into(staffs)
        .values(&staff)
        .get_result(conn)
}

pub fn get_all_staff(conn: &PgConnection) -> QueryResult<Vec<Staff>> {
    info!("   Getting all staff");
    staffs.load::<Staff>(conn)
}

pub fn get_staff(staff_id: i32, conn: &PgConnection) -> QueryResult<Staff> {
    info!("   Getting staff by id");
    staffs.find(staff_id).get_result::<Staff>(conn)
}

pub fn get_all_staff_by_faculty(faculty_id: i32, conn: &PgConnection) -> QueryResult<Vec<Staff>> {
    info!("   Getting staff by faculty id");
    staffs.filter(faculty.eq(faculty_id)).get_results::<Staff>(conn)
}

pub fn update_staff(staff_id: i32, staff: Staff, conn: &PgConnection) -> QueryResult<Staff> {
    info!("{}", format!("   Updating staff {} {}", &staff.name, &staff.surname));
    diesel::update(staffs.find(staff_id))
        .set(&staff)
        .get_result(conn)
}

pub fn delete_staff(staff_id: i32, conn: &PgConnection) -> QueryResult<usize> {
    info!("   Deleting lecture from database");

    let old = staffs.load::<Staff>(conn).unwrap();
   
    diesel::delete(staffs.filter(id.eq(staff_id))).execute(conn)?;
    let new = staffs.load::<Staff>(conn).unwrap();

    if old.len() >= 1 && old.len() - 1 == new.len() {
        Ok(new.len())
    }else{
        Err(Error::NotFound)
    }
}