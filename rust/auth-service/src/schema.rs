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

table! {
    admins (id) {
        id -> Int4,
        admin_email -> Varchar,
        password -> Varchar,
        role -> Varchar,
    }
}