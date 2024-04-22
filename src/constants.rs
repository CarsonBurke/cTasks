// Figure out a way to reconcile spacings to work for use-cases and not just size names
pub mod padding {
    pub const SECTION: u16 = 22;
    pub const MAIN: u16 = 10;
    pub const PORTION: u16 = 6;
}

pub mod spacings {
    pub const XSMALL: u16 = 4;
    pub const SMALL: u16 = 8;
    pub const MEDIUM: u16 = 16;
    pub const LARGE: u16 = 32;
    pub const X_LARGE: u16 = 64;
}

pub mod font_sizes {
    pub const H1: u16 = 17;
    pub const H2: u16 = 15;
    pub const H3: u16 = 13;
    pub const P: u16 = 13;
}

pub mod sizings {
    pub const MAX_MAIN_CONTENT_CHILDREN_WIDTH: u16 = 800;
    pub const DEFAULT_CHART_HEIGHT: f32 = 300.;
}

#[derive(Debug, Default, Copy, Clone)]
pub enum DisplayState {
    #[default]
    Shown,
    Hidden,
}

pub const BYTE_UNITS_DECIMAL: [&str; 9] =
    ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"];
pub const BYTE_UNITS_BINARY: [&str; 9] =
    ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"];
pub const FREQUENCY_UNITS: [&str; 3] = ["MHz", "GHz", "THz"];
pub static ICON: &[u8] = include_bytes!("icon.png");

/// How many decimals to show for percentage values.
/// TODO: Make this user configurable
pub const PERCENT_PRECISION: u8 = 1;
/// How many ticks to keep contigious history for
// TODO: Make this user configurable
pub const HISTORY_TICKS: u32 = 30;

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
        r: 0.20,
        g: 0.20,
        b: 0.20,
        a: 1.,
    };

    pub const BACKGROUND_4: Color = Color {
        r: 0.24,
        g: 0.24,
        b: 0.24,
        a: 1.,
    };

    pub const BACKGROUND_5: Color = Color {
        r: 0.28,
        g: 0.28,
        b: 0.28,
        a: 1.,
    };

    pub const TEXT: Color = Color {
        r: 1.,
        g: 1.,
        b: 1.,
        a: 1.,
    };

    pub const GREY_TEXT: Color = Color {
        r: 0.7,
        g: 0.7,
        b: 0.7,
        a: 1.,
    };
}
