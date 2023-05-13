use crate::enums::days;
use std::collections::HashMap;

pub type Schedule = Vec<AssignedShifts>;

pub trait GetSchedule {
    fn get_copy(&self) -> Schedule;
}

//impl Schedule {
impl GetSchedule for Schedule {
    fn get_copy(&self) -> Schedule {
        let mut schedule_copy = Vec::new();
    
        for assigned_shift in self {
            let mut lectures_copy = Vec::new();
            for lecture in &assigned_shift.lectures {
                lectures_copy.push(Lecture{name:(*lecture.name).to_owned(), number_of_times: lecture.number_of_times});
            }
    
            let mut shifts_copy: ShiftMap = HashMap::new();
    
            for (k, v) in &assigned_shift.shifts {
                let mut v_copy: Vec<Vertex> = Vec::new();
    
                for vertex in v {
                    v_copy.push(Vertex{start: (*vertex.start).to_owned(), end: (*vertex.end).to_owned()});
                }
                shifts_copy.insert((*k).to_owned(), v_copy);
            }
            schedule_copy.push(AssignedShifts{
                staff_id: assigned_shift.staff_id,
                staff: (*assigned_shift.staff).to_owned(),
                lectures: lectures_copy,
                shifts: shifts_copy
            });
        }
        return schedule_copy;
    }
}

/// The genotype
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash)]
pub struct Chromosome {
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

#[derive(Serialize, Deserialize, Clone)]
pub struct AssignedShifts {
    pub staff_id: i32,
    pub staff: String,
    pub lectures: Vec<Lecture>,
    pub shifts: ShiftMap
}
pub type ShiftMap = HashMap<days::Day, Vec<Vertex>>;

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize, Clone)]
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