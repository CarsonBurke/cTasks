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

pub mod custom_theme {
    use iced::Color;

    pub const PRIMARY: Color = Color {
        r: 0.21,
        g: 0.52,
        b: 0.89,
        a: 1.,
    };

    pub const BACKGROUND_1: Color = Color {
        r: 0.12,
        g: 0.12,
        b: 0.12,
        a: 1.,
    };

    pub const BACKGROUND_2: Color = Color {
        r: 0.16,
        g: 0.16,
        b: 0.16,
        a: 1.,
    };

    pub const BACKGROUND_3: Color = Color {
        r: 0.25,
        g: 0.25,
        b: 0.25,
        a: 1.,
    };

    pub const TEXT: Color = Color {
        r: 1.,
        g: 1.,
        b: 1.,
        a: 1.,
    };
}