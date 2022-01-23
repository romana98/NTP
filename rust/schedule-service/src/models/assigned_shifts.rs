use crate::schema::assigned_shifts;
use crate:: models::algorithm::{ShiftLectureMap};

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct AssignedShift {
    pub id: i32,
    pub schedule_id: i32,
    pub staff_id: i32,
    pub staff: String
}
#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "assigned_shifts"]
pub struct NewAssignedShift {
    pub schedule_id: i32,
    pub staff_id: i32,
    pub staff: String
}

#[derive(Serialize, Deserialize)]
pub struct AssignedShiftDTO {
    pub staff: String,
    pub shifts: ShiftLectureMap
}
