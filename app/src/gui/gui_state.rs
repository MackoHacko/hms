use super::widgets::SnipListState;
use anyhow::{Ok, Result};
use hms_common::app_dir_client::AppDirClient;
use hms_db::{manager::HmsDbManager, models::Snip};

#[derive(Debug)]
pub struct GuiState<'a, P>
where
    P: AppDirClient,
{
    db_manager: &'a HmsDbManager<'a, P>,
    snip_limit: i64,
    query: String,
    pub list_state: SnipListState,
}

impl<'a, P> GuiState<'a, P>
where
    P: AppDirClient,
{
    pub fn new(db_manager: &'a HmsDbManager<'a, P>, snip_limit: i64) -> Result<Self> {
        let query = String::default();
        let snips =
            db_manager.with_db(|db| db.find_snips_by_alias(query.as_str(), snip_limit, 0))?;
        let list_state = SnipListState::new(snips);
        Ok(Self {
            db_manager,
            snip_limit,
            query: query,
            list_state,
        })
    }

    pub fn append_query(&mut self, c: char) -> Result<()> {
        self.query.push(c);
        Ok(self.refresh_snips()?)
    }

    pub fn pop_query(&mut self) -> Result<()> {
        self.query.pop();
        Ok(self.refresh_snips()?)
    }

    pub fn query(&self) -> &String {
        &self.query
    }

    fn get_snips(&mut self, offset: i64) -> Result<Vec<Snip>> {
        let snips = self
            .db_manager
            .with_db(|db| db.find_snips_by_alias(&self.query, self.snip_limit, offset))?;
        Ok(snips)
    }

    pub fn paginate(&mut self) -> Result<()> {
        if self.list_state.needs_next_page() {
            let next = self.get_snips(self.list_state.selected_snip_index() as i64)?;
            self.list_state.extend_snips(next);
        }
        Ok(())
    }

    fn refresh_snips(&mut self) -> Result<()> {
        let snips = self.get_snips(0)?;
        self.list_state.set_snips(snips);
        Ok(())
    }
}
