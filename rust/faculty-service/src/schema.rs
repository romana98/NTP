table! {
    faculties (id) {
        id -> Int4,
        name -> Varchar,
        hard_constraint_id -> Int4,
        schedule_id -> Nullable<Int4>,
    }
}

table! {
    hard_constraints (id) {
        id -> Int4,
        daily_max -> Int4,
        weekly_max -> Int4,
        weekly_min -> Int4,
        max_per_shift -> Int4,
    }
}

table! {
    lectures_faculties (id) {
        id -> Int4,
        faculty_id -> Int4,
        lecture_id -> Int4,
    }
}

table! {
    shifts (id) {
        id -> Int4,
        start_shift -> Varchar,
        end_shift -> Varchar,
        day -> Varchar,
    }
}

table! {
    shifts_faculties (id) {
        id -> Int4,
        faculty_id -> Int4,
        shift_id -> Int4,
    }
}

table! {
    staffs_faculties (id) {
        id -> Int4,
        faculty_id -> Int4,
        staff_id -> Int4,
    }
}

joinable!(faculties -> hard_constraints (hard_constraint_id));
joinable!(shifts_faculties -> shifts (shift_id));

allow_tables_to_appear_in_same_query!(
    faculties,
    hard_constraints,
    lectures_faculties,
    shifts,
    shifts_faculties,
    staffs_faculties,
);
