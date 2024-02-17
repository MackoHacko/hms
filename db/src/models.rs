use crate::schema::snips;
use chrono::NaiveDateTime;
use diesel::{Identifiable, Insertable, Queryable, Selectable};

#[derive(Queryable, Identifiable, Selectable, Debug)]
#[diesel(table_name = snips)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Snip {
    pub id: i32,
    pub alias: String,
    pub value: String,
    pub access_count: i32,
    pub created: NaiveDateTime,
    pub last_access: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = snips)]
pub struct NewSnip<'a> {
    pub alias: &'a str,
    pub value: &'a str,
}

impl<'a> NewSnip<'a> {
    pub fn new(alias: &'a str, value: &'a str) -> Self {
        NewSnip { alias, value }
    }
}
