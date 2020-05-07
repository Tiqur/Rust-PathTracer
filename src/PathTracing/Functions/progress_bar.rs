use console::style;

pub fn progress_bar(current: i32, maximum: i32, barsize: i32) -> String {
    let mut percent = current as f32/ (maximum as f32 - 1.0);
    let new = percent * barsize as f32;
    let mut bar1 = String::new();
    let mut bar2 = String::new();

    while bar1.len() + bar2.len() < barsize as usize {
        if bar1.len() + bar2.len() < new as usize {
            bar1 = format!("{}{}", bar1, "=")
        }
        if bar1.len() + bar2.len() >= new as usize && bar1.len() + bar2.len() < barsize as usize {
            bar2 = format!("{}{}", bar2, "-")
        }
    }
    percent = (percent * 100.0).round();

    let percent_str = if percent < 100.0 {
        style(percent.to_string()).yellow()
    } else {
        style(percent.to_string()).green()
    };
    return format!("[{}{}] {}%", style(bar1).blue(), bar2, percent_str)
}