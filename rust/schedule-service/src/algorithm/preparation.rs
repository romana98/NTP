use crate::{models::{algorithm::*}, enums::days::Day};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub fn prepare_data(faculty: &FacultySchedule, staff: &Vec<StaffSchedule>, lectures: &Vec<LectureSchedule>) -> (Option<Simulator>, i32) {

    let mut shift_map: ShiftMap = HashMap::new();
    for shift in &faculty.shifts {
        let key = &shift.day;

        match shift_map.entry(Day::from_str(&key)) {
            Entry::Vacant(e) => { e.insert(vec![Vertex{start: shift.start.to_owned(), end: shift.end.to_owned()}]); },
            Entry::Occupied(mut e) => { e.get_mut().push(Vertex{start: shift.start.to_owned(), end: shift.end.to_owned()}); }
        }
    }

    let mut lecture_len: i32 = 0;
    let mut staff_vec = Vec::new();
    let mut sc_by_staff: SCbyStaff = HashMap::new();

    for st in staff {
        let mut prefs = HashMap::new();
        for (k, v) in &st.soft_constraint.prefers {
            prefs.insert((*k).to_owned(), (*v).to_owned());
        }
        sc_by_staff.insert(st.id, SoftConstraint{id: (*st.soft_constraint.id).to_owned(), prefers: prefs});

        let mut lectures_vec = Vec::new();
        let mut num_of_shifts: i32 = 0;
        for lecture in &st.lectures {

           let lec_option = lectures.iter().find(|&x| x.id == *lecture);
           if lec_option.is_some(){
                let lec = lec_option.unwrap();
                let num = lec.number_of_times;
                num_of_shifts += num;

                lectures_vec.push(Lecture{name: (*lec.name).to_owned(), number_of_times: num});
            }
        }
        lecture_len +=  num_of_shifts;
        staff_vec.push(Staff{
            id: st.id,
            full_name: format!("{} {}", st.name, st.surname),
            lectures: lectures_vec,
            num_of_shifts: num_of_shifts
        });
    }

  (Some(Simulator{
        end_criteria: HardConstraint{
                        id: (*faculty.hard_constraint.id).to_owned(),
                        daily_max: faculty.hard_constraint.daily_max, 
                        weekly_max: faculty.hard_constraint.weekly_max, 
                        weekly_min: faculty.hard_constraint.weekly_min,
                        max_per_shift: faculty.hard_constraint.max_per_shift
                    },
        shift_map: shift_map,
        soft_constraints_by_staff: sc_by_staff,
        staff: staff_vec
    }), lecture_len)
}
