use crate::schema::shifts_lectures;

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct ShiftsLecture {
    pub id: i32,
    pub assigned_shifts_id: i32,
    pub day: String,
    pub shift: String,
    pub lecture: String
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "shifts_lectures"]
pub struct NewShiftsLecture {
    pub assigned_shifts_id: i32,
    pub day: String,
    pub shift: String,
    pub lecture: String
}

#[derive(Serialize, Deserialize)]
pub struct ShiftsLectureDTO {
    pub id: i32,
    pub assigned_shifts_id: i32,
    pub day: String,
    pub shift: String,
    pub lecture: String
}