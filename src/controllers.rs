use crate::{
    models::{Link, NewLink, NewUser, User},
    schema::links,
    schema::users,
};
use diesel::prelude::*;
pub struct LinkController;

impl LinkController {
    pub fn find_one(c: &mut SqliteConnection, id: i32) -> QueryResult<Link> {
        links::table::find(links::table, id).get_result::<Link>(c)
    }
    pub fn find_many(c: &mut SqliteConnection) -> QueryResult<Vec<Link>> {
        links::table
            .order(links::id.desc())
            .limit(1000)
            // .load::<Link>(c)
            .load(c)
    }

    pub fn create(c: &mut SqliteConnection, new_link: NewLink) -> QueryResult<Link> {
        diesel::insert_into(links::table)
            .values(new_link)
            .execute(c)?;
        let last_id = Self::last_id(c)?;
        Self::find_one(c, last_id)
    }

    pub fn update(c: &mut SqliteConnection, link: Link, id: i32) -> QueryResult<Link> {
        if Self::find_one(c, id).is_err() {
            Err(diesel::result::Error::NotFound)
        } else {
            diesel::update(links::table::find(links::table, id))
                .set(links::url.eq(link.url.to_owned()))
                .execute(c)?;

            Self::find_one(c, id)
        }
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

pub struct UserController;

impl UserController {
    pub fn find_one(c: &mut SqliteConnection, id: i32) -> QueryResult<User> {
        users::table::find(users::table, id).get_result::<User>(c)
    }

    pub fn find_many(c: &mut SqliteConnection) -> QueryResult<Vec<User>> {
        users::table.order(users::id.desc()).limit(1000).load(c)
    }

    pub fn create(c: &mut SqliteConnection, new_user: NewUser) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(new_user)
            .execute(c)?;

        let last_id = Self::last_id(c)?;
        Self::find_one(c, last_id)
    }

    pub fn update(c: &mut SqliteConnection, user: User, id: i32) -> QueryResult<User> {
        if Self::find_one(c, id).is_err() {
            Err(diesel::result::Error::NotFound)
        } else {
            diesel::update(users::table::find(users::table, id))
                .set((
                    users::name.eq(user.name.to_owned()),
                    users::email.eq(user.email.to_owned()),
                    users::password.eq(user.password.to_owned()),
                ))
                .execute(c)?;
            Self::find_one(c, id)
        }
    }

    pub fn delete(c: &mut SqliteConnection, id: i32) -> QueryResult<usize> {
        if Self::find_one(c, id).is_err() {
            return Err(diesel::result::Error::NotFound);
        }
        diesel::delete(users::table.find(id)).execute(c)
    }

    pub fn last_id(c: &mut SqliteConnection) -> QueryResult<i32> {
        users::table::select(users::table, users::id)
            .order(users::id.desc())
            .first(c)
    }
    pub fn find_user(c: &mut SqliteConnection, email: &String, password: &String) {
        let user_result = users::table
            .filter(users::email.eq(email.to_owned()))
            .filter(users::password.eq(password.to_owned()))
            .first::<User>(c);
        println!("{:?}", user_result);
    }
}

pub struct Verifier;

impl Verifier {
    pub fn verify(c: &mut SqliteConnection, email: &str, password: &str) -> QueryResult<User> {
        users::table
            .filter(users::email.eq(email))
            .filter(users::password.eq(password))
            .first(c)
    }
}
