pub fn format_bytes(mut bytes: f32) -> String {
    let units = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"];
    let mut i = 0;
    while bytes >= 1024. && i < units.len() - 1 {
        bytes /= 1024.;
        i += 1;
    }
    format!("{:.2} {}", bytes, units[i])
}

pub fn format_hz(mut hz: f32) -> String {
    let units = ["MHz", "GHz", "THz"];
    let mut i = 0;
    while hz >= 1000. && i < units.len() - 1 {
        hz /= 1000.;
        i += 1;
    }
    format!("{:.2} {}", hz, units[i])
}