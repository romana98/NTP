use crate::schema::lectures_staffs;

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct LecturesStaff {
    pub id: i32,
    pub staff_id: i32,
    pub lecture_id: i32
}

#[derive(Serialize, Deserialize)]
pub struct LecturesStaffDTO {
    pub id: i32,
    pub staff_id: i32,
    pub lecture_id: i32
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "lectures_staffs"]
pub struct NewLecturesStaff {
    pub staff_id: i32,
    pub lecture_id: i32
}

#[derive(Serialize, Deserialize)]
pub struct IdsDTO {
    pub ids: Vec<String>
}