pub mod padding {
    pub const MAIN: u16 = 10;
}

#[derive(Debug, Default)]
pub enum DisplayState {
    #[default]
    Shown,
    Hidden,
}