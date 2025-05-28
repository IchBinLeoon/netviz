pub fn format_bytes(bytes: f64, is_speed: bool) -> String {
    const KB: f64 = 1000.0;
    const MB: f64 = KB * 1000.0;
    const GB: f64 = MB * 1000.0;
    const TB: f64 = GB * 1000.0;

    let value = if bytes >= TB {
        format!("{:.2} TB", bytes / TB)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes / GB)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes / MB)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes / KB)
    } else {
        format!("{:.2} B", bytes)
    };

    if is_speed {
        format!("{}/s", value)
    } else {
        value
    }
}
