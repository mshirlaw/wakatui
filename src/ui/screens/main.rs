use chrono::Timelike;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::app::AppData;
use crate::ui::widgets::{Footer, Header, Projects, Statistics, Today, TodayData};

pub fn render_main_screen(frame: &mut Frame, data: &AppData) {
    let size = frame.area();

    let container = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(15),
            Constraint::Percentage(40),
            Constraint::Percentage(25),
            Constraint::Percentage(10),
        ])
        .split(size);

    render_header(frame, container[0]);
    render_statistics(frame, container[1], data);
    render_projects(frame, container[2]);
    render_today(frame, container[3]);
    render_footer(frame, container[4])
}

pub fn render_header(frame: &mut Frame, area: Rect) {
    frame.render_widget(Header, area);
}

pub fn render_statistics(frame: &mut Frame, area: Rect, app_data: &AppData) {
    let today_data = if let Some(summary) = &app_data.today_summary {
        let hourly_data = process_hourly_data(&app_data.today_durations);
        TodayData {
            hours: summary.grand_total.hours,
            minutes: summary.grand_total.minutes,
            hourly_data,
        }
    } else {
        TodayData::default()
    };

    frame.render_widget(Statistics::new(today_data), area);
}

fn process_hourly_data(durations: &Option<crate::api::DurationsResponse>) -> Vec<f64> {
    let mut hourly_buckets = vec![0.0; 12]; // 7am-7pm = 12 hours

    if let Some(durations_response) = durations {
        for duration in &durations_response.data {
            let datetime = chrono::DateTime::from_timestamp(duration.time as i64, 0);
            if let Some(dt_utc) = datetime {
                let dt_local: chrono::DateTime<chrono::Local> = dt_utc.into();
                let hour = dt_local.hour();

                if (7..19).contains(&hour) {
                    let bucket_index = (hour - 7) as usize;
                    hourly_buckets[bucket_index] += duration.duration;
                }
            }
        }
    }

    hourly_buckets
}

pub fn render_projects(frame: &mut Frame, area: Rect) {
    frame.render_widget(Projects, area);
}

pub fn render_today(frame: &mut Frame, area: Rect) {
    frame.render_widget(Today, area);
}

pub fn render_footer(frame: &mut Frame, area: Rect) {
    frame.render_widget(Footer, area);
}
