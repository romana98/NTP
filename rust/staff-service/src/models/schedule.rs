use crate::models::soft_constraint::SoftConstraintsDTO;

#[derive(Serialize, Deserialize)]
pub struct StaffSchedule {
    pub id: i32,
    pub name: String,
    pub surname: String,
    pub lectures: Vec<i32>,
    pub soft_constraint: SoftConstraintsDTO
}