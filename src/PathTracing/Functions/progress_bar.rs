pub fn progress_bar(current: i32, maximum: i32, barsize: i32) -> String {
    let percent = current as f32/ (maximum as f32 - 1.0);
    let new = percent * barsize as f32;
    let mut bar = String::from("");
    while bar.len() < barsize as usize {
        if bar.len() < new as usize {
            bar = format!("{}{}", bar, "=")
        }
        if bar.len() >= new as usize && bar.len() < barsize as usize {
            bar = format!("{}{}", bar, "-")
        }
    }
    return format!("[{}] {}%", bar, (percent * 100.0) as i32)
}