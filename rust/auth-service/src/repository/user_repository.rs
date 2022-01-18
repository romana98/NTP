use crate::{
    schema::{staffs::dsl::*, admins::dsl::*},
    models::user::{Staff, Admin}
};
use diesel::prelude::*;

pub fn get_staff_by_email(mail: &str, conn: &PgConnection) -> QueryResult<Staff> {
    info!("   Getting staff by email");
    staffs.filter(email.eq(mail)).get_result::<Staff>(conn)
}

pub fn get_admin_by_email(mail: &str, conn: &PgConnection) -> QueryResult<Admin> {
    info!("   Getting admin by email");
    admins.filter(admin_email.eq(mail)).get_result::<Admin>(conn)
}