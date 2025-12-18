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
        TodayData {
            hours: summary.grand_total.hours,
            minutes: summary.grand_total.minutes,
        }
    } else {
        TodayData::default()
    };

    frame.render_widget(Statistics::new(today_data), area);
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
