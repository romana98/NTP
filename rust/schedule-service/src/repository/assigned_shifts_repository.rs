use crate::{
    schema::assigned_shifts::dsl::*,
    models::assigned_shifts::{NewAssignedShift, AssignedShift},
    //repository::prefers_repository
};
use diesel::prelude::*;
//use diesel::result::Error;


pub fn create_asigned_shift(schedule_id_id: i32, staff_id_id: i32, staff_name: String, conn: &PgConnection) -> QueryResult<AssignedShift> {
    info!("{}", format!("   Inserting asigned_shift"));
    
    let asigned_shift = NewAssignedShift{schedule_id: schedule_id_id, staff_id: staff_id_id, staff: staff_name};

    diesel::insert_into(assigned_shifts)
        .values(&asigned_shift)
        .get_result(conn)
}