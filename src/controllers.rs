use crate::{
    models::{Link, NewLink},
    schema::links,
};
use diesel::prelude::*;

pub struct Controller;

impl Controller {
    pub fn find_one(c: &mut SqliteConnection, id: i32) -> QueryResult<Link> {
        links::table::find(links::table, id).get_result::<Link>(c)
    }
    pub fn find_many(c: &mut SqliteConnection) -> QueryResult<Vec<Link>> {
        links::table
            .order(links::id.desc())
            .limit(1000)
            .load::<Link>(c)
    }

    pub fn create(c: &mut SqliteConnection, new_link: NewLink) -> QueryResult<Link> {
        diesel::insert_into(links::table)
            .values(new_link)
            .execute(c)?;
        let last_id = Self::last_id(c)?;
        Self::find_one(c, last_id)
    }

    pub fn delete(c: &mut SqliteConnection, id: i32) -> QueryResult<usize> {
        if Self::find_one(c, id).is_err() {
            return Err(diesel::result::Error::NotFound);
        }
        diesel::delete(links::table.find(id)).execute(c)
    }

    pub fn last_id(c: &mut SqliteConnection) -> QueryResult<i32> {
        links::table::select(links::table, links::id)
            .order(links::id.desc())
            .first(c)
    }
}
