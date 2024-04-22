use crate::{
    constants::{BYTE_UNITS_BINARY, BYTE_UNITS_DECIMAL, FREQUENCY_UNITS},
    preferences::{ByteBase, Preferences},
};

pub fn format_bytes(preferences: &Preferences, mut bytes: f32) -> String {
    let (units, factor) = match preferences.byte_base {
        ByteBase::Decimal => {
            let units = BYTE_UNITS_DECIMAL.clone();
            let factor = 1000.;

            (units, factor)
        }
        ByteBase::Binary => {
            let units = BYTE_UNITS_BINARY.clone();
            let factor = 1024.;

            (units, factor)
        }
    };
    
    let mut i = 0;

    while bytes >= factor && i < units.len() - 1 {
        bytes /= factor;
        i += 1;
    }

    format!("{:.2} {}", bytes, units[i])
}

pub fn round_bytes_list(preferences: &Preferences, bytes_vec: Vec<f32>) -> (Vec<String>, String) {
    let (units, factor) = match preferences.byte_base {
        ByteBase::Decimal => {
            let units = BYTE_UNITS_DECIMAL.clone();
            let factor = 1000.;

            (units, factor)
        }
        ByteBase::Binary => {
            let units = BYTE_UNITS_BINARY.clone();
            let factor = 1024.;

            (units, factor)
        }
    };

    let mut min_unit: &str = "none";
    let mut min_multiplier: usize = usize::MAX;

    for bytes in &bytes_vec {
        let mut i: usize = 0;
        let mut test_bytes = bytes.clone();

        while test_bytes >= factor && i < units.len() - 1 {
            test_bytes /= factor;
            i += 1;
        }

        if i >= min_multiplier {
            continue;
        }

        min_unit = units[i];
        min_multiplier = i;
    }

    println!("min multiplier {}", min_multiplier);    

    let mut modified_bytes_list: Vec<String> = Vec::new();

    for bytes in bytes_vec {
        let modified_bytes = format!("{:.2} {}", bytes / factor.powf(min_multiplier as f32), min_unit);
        modified_bytes_list.push(modified_bytes);
    };

    (modified_bytes_list, min_unit.to_string())
}

pub fn format_hz(preferences: &Preferences, mut hz: f32) -> String {
    let units = FREQUENCY_UNITS.clone();
    let mut i = 0;
    while hz >= 1000. && i < units.len() - 1 {
        hz /= 1000.;
        i += 1;
    }
    format!("{:.2} {}", hz, units[i])
}