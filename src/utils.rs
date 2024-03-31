pub fn format_bytes(mut bytes: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    let mut i = 0;
    while bytes >= 1024 && i < units.len() - 1 {
        bytes /= 1024;
        i += 1;
    }
    format!("{} {}", bytes, units[i])
}