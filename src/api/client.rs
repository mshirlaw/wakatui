use base64::{Engine as _, engine::general_purpose::STANDARD};
use color_eyre::eyre::{Result, eyre};
use reqwest::blocking::Client;

use super::models::{DurationsResponse, StatsResponse, SummariesResponse};

const API_BASE: &str = "https://api.wakatime.com/api/v1";

#[derive(Debug, Clone)]
pub struct ApiClient {
    client: Client,
    api_key: String,
}

impl ApiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    fn auth_header(&self) -> String {
        format!("Basic {}", STANDARD.encode(self.api_key.as_bytes()))
    }

    pub fn get_summaries(&self, start_date: &str, end_date: &str) -> Result<SummariesResponse> {
        let url = format!(
            "{}/users/current/summaries?start={}&end={}",
            API_BASE, start_date, end_date
        );

        let response = self
            .client
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()?;

        if !response.status().is_success() {
            return Err(eyre!("Failed to fetch summaries: {}", response.status()));
        }

        let summaries: SummariesResponse = response.json()?;
        Ok(summaries)
    }

    #[allow(dead_code)]
    pub fn get_stats(&self, range: &str) -> Result<StatsResponse> {
        let url = format!("{}/users/current/stats/{}", API_BASE, range);

        let response = self
            .client
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()?;

        if !response.status().is_success() {
            return Err(eyre!("Failed to fetch stats: {}", response.status()));
        }

        let stats: StatsResponse = response.json()?;
        Ok(stats)
    }

    pub fn get_today(&self) -> Result<SummariesResponse> {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        self.get_summaries(&today, &today)
    }

    #[allow(dead_code)]
    pub fn get_last_7_days(&self) -> Result<SummariesResponse> {
        let end = chrono::Local::now();
        let start = end - chrono::Duration::days(6);

        let start_date = start.format("%Y-%m-%d").to_string();
        let end_date = end.format("%Y-%m-%d").to_string();

        self.get_summaries(&start_date, &end_date)
    }

    pub fn get_durations(&self, date: &str) -> Result<DurationsResponse> {
        let url = format!("{}/users/current/durations?date={}", API_BASE, date);

        let response = self
            .client
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()?;

        if !response.status().is_success() {
            return Err(eyre!("Failed to fetch durations: {}", response.status()));
        }

        let durations: DurationsResponse = response.json()?;
        Ok(durations)
    }

    pub fn get_today_durations(&self) -> Result<DurationsResponse> {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        self.get_durations(&today)
    }
}
