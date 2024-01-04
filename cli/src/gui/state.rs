use super::widgets::snip_list::SnipListState;

#[derive(Debug, Default)]
pub struct GuiState {
    pub query: String,
    pub list_state: SnipListState,
}
