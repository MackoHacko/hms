use crate::{
    models::{NewSnip, Snip},
    prelude::*,
    schema::snips::dsl::*,
};
use chrono::Utc;
use diesel::{insert_into, prelude::*, update};

pub struct HmsDb<'a> {
    pub conn: &'a mut SqliteConnection,
}

impl<'a> HmsDb<'a> {
    pub fn insert_snip(&mut self, new_snip: &NewSnip) -> Result<Snip> {
        insert_into(snips)
            .values(new_snip)
            .returning(Snip::as_returning())
            .get_result(self.conn)
            .map_err(From::from)
    }

    pub fn find_snip_by_id(&mut self, snip_id: i32) -> Result<Snip> {
        snips
            .find(snip_id)
            .get_result(self.conn)
            .map_err(From::from)
    }

    pub fn find_snips_by_alias(
        &mut self,
        snip_alias: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Snip>> {
        snips
            .filter(alias.like(format!("%{}%", snip_alias)))
            .limit(limit)
            .offset(offset)
            .load::<Snip>(self.conn)
            .map_err(From::from)
    }

    pub fn increment_snip_access_count(&mut self, snip: &Snip) -> Result<()> {
        let now = Utc::now().naive_utc();
        update(snip)
            .set((access_count.eq(access_count + 1), last_access.eq(now)))
            .execute(self.conn)?;
        Ok(())
    }
}
