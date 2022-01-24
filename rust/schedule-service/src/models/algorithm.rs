use crate::enums::days;
use std::collections::HashMap;


/// The phenotype
pub type Schedule = Vec<AssignedShifts>;

/// The genotype
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Chromosome 
{
    pub day_index: i8
}

pub type Chromosomes = Vec<Chromosome>;


#[derive(Serialize, Deserialize)]
pub struct Simulator {
    pub shift_map: ShiftMap,
    pub staff: Vec<Staff>,
    pub soft_constraints_by_staff: SCbyStaff,
    pub end_criteria: HardConstraint
}

pub type ShiftLectureMap = HashMap<days::Day, VertexLecture>;

#[derive(Serialize, Deserialize)]
pub struct VertexLecture {
    pub start_shift: String,
    pub end_shift: String,
    pub lecture: String
}

#[derive(Serialize, Deserialize)]
pub struct AssignedShifts {
    pub staff_id: i32,
    pub staff: String,
    pub lectures: Vec<Lecture>,
    pub shifts: ShiftMap
}
pub type ShiftMap = HashMap<days::Day, Vec<Vertex>>;

#[derive(Serialize, Deserialize)]
pub struct Vertex {
    pub start: String,
    pub end: String,
}

#[derive(Serialize, Deserialize)]
pub struct SoftConstraint {
    pub id: String,
    pub prefers: HashMap<String, i32>
}

pub type SCbyStaff =  HashMap<i32, SoftConstraint>;


#[derive(Serialize, Deserialize)]
pub struct Lecture {
    pub name: String,
    pub number_of_times: i32
}

#[derive(Serialize, Deserialize)]
pub struct Staff {
    pub id: i32,
    pub full_name: String,
    pub lectures: Vec<Lecture>,
    pub num_of_shifts: i32
}

#[derive(Serialize, Deserialize)]
pub struct IdDTO {
    pub id: String
}

#[derive(Serialize, Deserialize)]
pub struct FacultySchedule {
    pub id: i32,
    pub schedule_id: i32,
    pub name: String,
    pub hard_constraint: HardConstraint,
    pub shifts: Vec<Shift>,
    pub staff: Vec<i32>,
    pub lectures: Vec<i32>
}

#[derive(Serialize, Deserialize)]
pub struct HardConstraint {
    pub id: String,
    pub daily_max: i32,
    pub weekly_max: i32,
    pub weekly_min: i32,
    pub max_per_shift: i32 
}

#[derive(Serialize, Deserialize)]
pub struct Shift {
    pub id: String,
    pub start: String,
    pub end: String,
    pub day: String
}

#[derive(Serialize, Deserialize)]
pub struct StaffSchedule {
    pub id: i32,
    pub name: String,
    pub surname: String,
    pub lectures: Vec<i32>,
    pub soft_constraint: SoftConstraint
}

#[derive(Serialize, Deserialize)]
pub struct LectureSchedule {
    pub id: i32,
    pub name: String,
    pub number_of_times: i32
}

#[derive(Serialize, Deserialize)]
pub struct FacultyDTO {
    pub faculty_id: i32,
    pub schedule_id: i32
}