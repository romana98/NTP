use crate::{
    schema::soft_constraints::dsl::*,
    models::soft_constraint::{NewSoftConstraints, SoftConstraint},
    repository::prefers_repository
};
use diesel::prelude::*;
use diesel::result::Error;


pub fn create_soft_constraints(conn: &PgConnection) -> QueryResult<SoftConstraint> {
    info!("{}", format!("   Inserting soft_constraints"));
    
    let sc = NewSoftConstraints{id: None};

    diesel::insert_into(soft_constraints)
        .values(&sc)
        .get_result(conn)
}

pub fn get_soft_constraints(sc_id: i32, conn: &PgConnection) -> QueryResult<SoftConstraint> {
    info!("{}", format!("   Getting soft_constraints"));
    soft_constraints.find(sc_id).get_result::<SoftConstraint>(conn)
}

pub fn delete_soft_constraints(sc_id: i32, conn: &PgConnection) -> QueryResult<usize> {
    info!("   Deleting soft_constraints from database");

    prefers_repository::delete_prefers(sc_id, conn)?;

    let old = soft_constraints.load::<SoftConstraint>(conn).unwrap();
   
    diesel::delete(soft_constraints.filter(id.eq(sc_id))).execute(conn)?;
    let new = soft_constraints.load::<SoftConstraint>(conn).unwrap();

    if old.len() >= 1 && old.len() - 1 == new.len() {
        Ok(new.len())
    }else{
        Err(Error::NotFound)
    }
}