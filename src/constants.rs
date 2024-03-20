pub mod padding {
    pub const MAIN: u16 = 10;
}

#[derive(Debug, Default)]
pub enum DisplayState {
    #[default]
    Shown,
    Hidden,
}

/// How many decimals to show for percentage values.
/// TODO: Make this user configurable
pub const PERCENT_PRECISION: u8 = 1;