use std::env;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref LECTURE_SERVICE: String = env::var("LECTURE_SERVICE").expect("LECTURE_SERVICE not found.");
}

lazy_static! {
    pub static ref STAFF_SERVICE: String = env::var("STAFF_SERVICE").expect("STAFF_SERVICE not found.");
}

lazy_static! {
    pub static ref FACULTY_SERVICE: String = env::var("FACULTY_SERVICE").expect("FACULTY_SERVICE not found.");
}

lazy_static! {
    pub static ref SCHEDULE_SERVICE: String = env::var("SCHEDULE_SERVICE").expect("SCHEDULE_SERVICE not found.");
}

lazy_static! {
    pub static ref AUTH_SERVICE: String = env::var("AUTH_SERVICE").expect("AUTH_SERVICE not found.");
}