table! {
    assigned_shifts (id) {
        id -> Int4,
        schedule_id -> Int4,
        staff_id -> Int4,
        staff -> Varchar,
    }
}

table! {
    schedules (id) {
        id -> Int4,
    }
}

table! {
    shifts_lectures (id) {
        id -> Int4,
        assigned_shifts_id -> Int4,
        day -> Varchar,
        shift -> Varchar,
        lecture -> Varchar,
    }
}

joinable!(assigned_shifts -> schedules (schedule_id));
joinable!(shifts_lectures -> assigned_shifts (assigned_shifts_id));

allow_tables_to_appear_in_same_query!(
    assigned_shifts,
    schedules,
    shifts_lectures,
);
