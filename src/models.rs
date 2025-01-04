use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::vertices)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Vertex {
    pub id: i32,
    pub payload: String,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::edges)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Edge {
    pub start: i32,
    pub finish: i32,
    pub payload: String,
}
