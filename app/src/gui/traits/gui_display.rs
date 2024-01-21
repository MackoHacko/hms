use crate::gui::gui_state::GuiState;
use anyhow::Result;
use hms_common::app_dir_client::AppDirClient;
use std::{fmt::Debug, u16};

pub trait GuiDisplay<P>: Debug
where
    P: AppDirClient,
{
    fn new() -> Result<Self>
    where
        Self: Sized;
    fn resize(&mut self, w: u16, h: u16);
    fn update(&mut self, state: &mut GuiState<P>) -> Result<()>;
    fn clear(&mut self) -> Result<()>;
}
