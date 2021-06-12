#[derive(Deserialize, Debug)]
pub struct TimeStats {
    time_estimate: i32,
    total_time_spent: i32,
    human_time_estimate: Option<String>,
    human_total_time_spent: Option<String>,
}
