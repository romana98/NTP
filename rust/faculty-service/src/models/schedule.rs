use crate::{models::{hard_constraints::HardConstraintDTO, shift::ShiftDTO}};

#[derive(Serialize, Deserialize)]
pub struct FacultySchedule {
    pub id: i32,
    pub name: String,
    pub hard_constraint: HardConstraintDTO,
    pub shifts: Vec<ShiftDTO>,
    pub staff: Vec<i32>,
    pub lectures: Vec<i32>
}