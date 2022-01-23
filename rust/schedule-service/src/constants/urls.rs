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

pub const SCHEDULE_ID : &str = "/schedule/";
pub const SCHEDULE : &str = "/schedule";
