table! {
    admins (id) {
        id -> Int4,
        admin_email -> Varchar,
        password -> Varchar,
        role -> Varchar,
    }
}

table! {
    lectures_staffs (id) {
        id -> Int4,
        staff_id -> Int4,
        lecture_id -> Int4,
    }
}

table! {
    prefers (id) {
        id -> Int4,
        soft_c_id -> Int4,
        day -> Varchar,
        num -> Int4,
    }
}

table! {
    soft_constraints (id) {
        id -> Int4,
    }
}

table! {
    staffs (id) {
        id -> Int4,
        name -> Varchar,
        surname -> Varchar,
        email -> Varchar,
        password -> Varchar,
        soft_constraints -> Int4,
        faculty -> Int4,
        role -> Varchar,
    }
}

joinable!(lectures_staffs -> staffs (staff_id));
joinable!(prefers -> soft_constraints (soft_c_id));

allow_tables_to_appear_in_same_query!(
    admins,
    lectures_staffs,
    prefers,
    soft_constraints,
    staffs,
);
