use super::schema::*;

#[derive(Queryable)]
pub struct DbUser {
    pub id: i32,
    pub username: String,
    pub passhash: String,
}

#[derive(Insertable)]
#[diesel(table_name=users)]
pub struct InsUser {
    pub username: String,
    pub passhash: String,
}
