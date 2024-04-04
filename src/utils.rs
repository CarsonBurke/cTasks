use crate::{constants::{byte_units_binary, byte_units_decimal, frequency_units}, preferences::{ByteBase, Preferences}};

pub fn format_bytes(preferences: &Preferences, mut bytes: f32) -> String {

    let mut i = 0;

    let (units, factor) = match preferences.byte_base {
        ByteBase::Decimal => {
            let units = byte_units_decimal.clone();
            let factor = 1000.;

            (units, factor)
        }
        ByteBase::Binary => {   
            let units = byte_units_binary.clone();
            let factor = 1024.;

            (units, factor)
        }
    };

    while bytes >= factor && i < units.len() - 1 {
        bytes /= factor;
        i += 1;
    };

    format!("{:.2} {}", bytes, units[i])
}

pub fn format_hz(preferences: &Preferences, mut hz: f32) -> String {
    let units = frequency_units.clone();
    let mut i = 0;
    while hz >= 1000. && i < units.len() - 1 {
        hz /= 1000.;
        i += 1;
    }
    format!("{:.2} {}", hz, units[i])
}