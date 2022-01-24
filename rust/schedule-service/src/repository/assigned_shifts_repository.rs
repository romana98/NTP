use crate::{
    schema::assigned_shifts::dsl::*,
    models::assigned_shifts::{NewAssignedShift, AssignedShift}
};
use diesel::prelude::*;
use diesel::result::Error;


pub fn create_asigned_shift(schedule_id_id: i32, staff_id_id: i32, staff_name: String, conn: &PgConnection) -> QueryResult<AssignedShift> {
    info!("{}", format!("   Inserting asigned_shift"));
    
    let asigned_shift = NewAssignedShift{schedule_id: schedule_id_id, staff_id: staff_id_id, staff: staff_name};

    diesel::insert_into(assigned_shifts)
        .values(&asigned_shift)
        .get_result(conn)
}

pub fn get_asigned_shift_by_schedule_id(schedule_id_id: i32, conn: &PgConnection) -> QueryResult<Vec<AssignedShift>> {
    info!("{}", format!("   Getting asigned_shift by schedule_id"));
    assigned_shifts.filter(schedule_id.eq(schedule_id_id)).load::<AssignedShift>(conn)
}

pub fn get_asigned_shift_by_staff_id(staff_id_id: i32, conn: &PgConnection) -> QueryResult<Vec<AssignedShift>> {
    info!("{}", format!("   Getting asigned_shift by staff_id_id"));
    assigned_shifts.filter(staff_id.eq(staff_id_id)).load::<AssignedShift>(conn)
}

pub fn delete_asigned_shift(schedule_id_id: i32, conn: &PgConnection) -> QueryResult<usize> {
    info!("   Deleting asigned_shift from database");

    let old = assigned_shifts.load::<AssignedShift>(conn).unwrap();
    let num = assigned_shifts.filter(schedule_id.eq(schedule_id_id)).load::<AssignedShift>(conn).unwrap().len();
   
    diesel::delete(assigned_shifts.filter(schedule_id.eq(schedule_id_id))).execute(conn)?;
    let new = assigned_shifts.load::<AssignedShift>(conn).unwrap();

    if old.len() >= num && old.len() - num == new.len() {
        Ok(new.len())
    }else{
        Err(Error::NotFound)
    }
}