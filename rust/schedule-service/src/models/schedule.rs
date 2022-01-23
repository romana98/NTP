use crate::schema::schedules;
//use std::collections::HashMap;

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct Schedule {
    pub id: i32
}

#[derive(Serialize, Deserialize)]
pub struct ScheduleDTO {
    pub staff: String,
    pub day: String,
    pub lecture: String,
    pub shift: String
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "schedules"]
pub struct NewSchedule {
    pub id: Option<i32>
}