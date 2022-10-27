fn format_row(t: &str, m: &str, w: &str, d: &str, l: &str, p: &str) -> String {
    format!(
        "{:<30} |{:^4}| {:^3}| {:^3}| {:^3}| {:>2}",
        t, m, w, d, l, p
    )
}

pub fn tally(match_results: &str) -> String {
    let mut table = format_row("Team", "MP", "W", "D", "L", "P");
    table
}
