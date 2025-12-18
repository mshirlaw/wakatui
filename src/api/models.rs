use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummariesResponse {
    pub data: Vec<DailySummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailySummary {
    pub grand_total: GrandTotal,
    pub range: DateRange,
    pub projects: Vec<Project>,
    pub languages: Vec<Language>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrandTotal {
    pub digital: String,
    pub hours: i32,
    pub minutes: i32,
    pub text: String,
    pub total_seconds: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub date: String,
    pub start: String,
    pub end: String,
    pub text: String,
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub total_seconds: f64,
    pub percent: f64,
    pub digital: String,
    pub text: String,
    pub hours: i32,
    pub minutes: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Language {
    pub name: String,
    pub total_seconds: f64,
    pub percent: f64,
    pub digital: String,
    pub text: String,
    pub hours: i32,
    pub minutes: i32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsResponse {
    pub data: Stats,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub daily_average: f64,
    pub daily_average_including_other_language: f64,
    pub best_day: Option<BestDay>,
    pub human_readable_daily_average: String,
    pub human_readable_daily_average_including_other_language: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BestDay {
    pub date: String,
    pub text: String,
    pub total_seconds: f64,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentUser {
    pub data: UserData,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserData {
    pub id: String,
    pub email: Option<String>,
    pub username: Option<String>,
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DurationsResponse {
    pub data: Vec<Duration>,
    pub start: String,
    pub end: String,
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Duration {
    pub project: Option<String>,
    pub time: f64,
    pub duration: f64,
    #[serde(default)]
    pub ai_additions: Option<i32>,
    #[serde(default)]
    pub ai_deletions: Option<i32>,
    #[serde(default)]
    pub human_additions: Option<i32>,
    #[serde(default)]
    pub human_deletions: Option<i32>,
}
