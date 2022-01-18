use crate::{
    schema::hard_constraints::dsl::*,
    models::hard_constraints::{HardConstraint, NewHardConstraint}
};
use diesel::prelude::*;
use diesel::result::Error;

pub fn create_hard_constraint(hard_constraint: NewHardConstraint, conn: &PgConnection) -> QueryResult<HardConstraint> {
    info!("{}", format!("   Inserting hard_constraint"));
    diesel::insert_into(hard_constraints)
        .values(&hard_constraint)
        .get_result(conn)
}

pub fn get_hard_constraint(hard_constraint_id: i32, conn: &PgConnection) -> QueryResult<HardConstraint> {
    info!("   Getting hard_constraint by id");
    hard_constraints.find(hard_constraint_id).get_result::<HardConstraint>(conn)
}

pub fn update_hard_constraint(hard_constraint_id: i32, hard_constraint: HardConstraint, conn: &PgConnection) -> QueryResult<HardConstraint> {
    info!("{}", format!("   Updating hard_constraint {}", &hard_constraint_id));
    diesel::update(hard_constraints.find(hard_constraint_id))
        .set(&hard_constraint)
        .get_result(conn)
}

pub fn get_all_hard_constraints(conn: &PgConnection) -> QueryResult<Vec<HardConstraint>> {
    info!("  Gettinng all hard_constraints");
    hard_constraints.load::<HardConstraint>(conn)
}


pub fn delete_hard_constraint(hard_constraint_id: i32, conn: &PgConnection) -> QueryResult<usize> {
    info!("   Deleting hard_constraint from database");

    let old = hard_constraints.load::<HardConstraint>(conn).unwrap();
   
    diesel::delete(hard_constraints.filter(id.eq(hard_constraint_id))).execute(conn)?;
    let new = hard_constraints.load::<HardConstraint>(conn).unwrap();

    if old.len() >= 1 && old.len() - 1 == new.len() {
        Ok(new.len())
    }else{
        Err(Error::NotFound)
    }
}