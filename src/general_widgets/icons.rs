use iced::{widget::{text, Text}, Renderer, Theme};
use iced_aw::BootstrapIcon;

pub fn bootstrap_icon<'a>(icon_name: BootstrapIcon) -> Text<'a>/* -> Text<'a, Message> */ {
    text(icon_name).font(iced_aw::BOOTSTRAP_FONT)
}

pub fn battery_icon<'a>(battery_state: battery::State) -> Text<'a> {
    bootstrap_icon(match battery_state {
        battery::State::Charging => BootstrapIcon::BatteryCharging,
        battery::State::Discharging => BootstrapIcon::BatteryHalf,
        battery::State::Empty => BootstrapIcon::Battery,
        battery::State::Full => BootstrapIcon::BatteryFull,
        battery::State::Unknown => BootstrapIcon::Battery,
        _ => BootstrapIcon::Battery
    })
}