use crate::schema::links;
use diesel::Insertable;
use diesel::Queryable;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Debug, Queryable, Deserialize)]
pub struct Link {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub url: String,
    #[serde(skip_deserializing)]
    pub created_at: String,
}

#[derive(Deserialize, Debug, Insertable)]
#[diesel(table_name = links)]
pub struct NewLink {
    pub url: String,
}
