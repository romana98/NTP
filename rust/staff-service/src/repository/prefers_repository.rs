use crate::{
    schema::prefers::dsl::*,
    models::{
        prefers::{NewPrefers, Prefer}
    },
    enums::days
};
use diesel::prelude::*;
use diesel::result::Error;
use strum::IntoEnumIterator;

pub fn create_prefers(sc_id: i32, conn: &PgConnection) -> QueryResult<Vec<Prefer>> {
    info!("{}", format!("   Inserting soft_constraints"));
    let mut prefers_vec = Vec::new();

    for enum_day in days::Day::iter() {
        let prefer = NewPrefers{
            soft_c_id: sc_id,
            day: enum_day.to_string(),
            num: 1
        };
        prefers_vec.push(prefer);
    }
    diesel::insert_into(prefers)
        .values(&prefers_vec)
        .get_results(conn)
}

pub fn update_prefers(sc_id:i32, prefers_vec: Vec<NewPrefers>, conn: &PgConnection) -> QueryResult<Vec<Prefer>> {
    info!("{}", format!("   Updating prefers"));

    delete_prefers(sc_id, conn)?;
    
    diesel::insert_into(prefers)
    .values(&prefers_vec)
    .get_results(conn)
}

pub fn get_prefers(sc_id: i32, conn: &PgConnection) -> QueryResult<Vec<Prefer>> {
    info!("{}", format!("   Getting soft_constraints"));
    prefers.filter(soft_c_id.eq(sc_id)).load::<Prefer>(conn)
}

pub fn delete_prefers(sc_id: i32, conn: &PgConnection) -> QueryResult<usize> {
    info!("   Deleting soft_constraints from database");

    let old = prefers.load::<Prefer>(conn).unwrap();
   
    diesel::delete(prefers.filter(soft_c_id.eq(sc_id))).execute(conn)?;
    let new = prefers.load::<Prefer>(conn).unwrap();

    if  old.len() >= 5 && old.len() - 5 == new.len() {
        Ok(new.len())
    }else{
        Err(Error::NotFound)
    }
}

