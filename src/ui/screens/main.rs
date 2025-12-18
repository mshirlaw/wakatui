use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::ui::widgets::{Footer, Header, Projects, Statistics, Today};

pub fn render_main_screen(frame: &mut Frame) {
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
    render_statistics(frame, container[1]);
    render_projects(frame, container[2]);
    render_today(frame, container[3]);
    render_footer(frame, container[4])
}

pub fn render_header(frame: &mut Frame, area: Rect) {
    frame.render_widget(Header, area);
}

pub fn render_statistics(frame: &mut Frame, area: Rect) {
    frame.render_widget(Statistics, area);
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
