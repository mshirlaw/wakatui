use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

pub struct TodayData {
    pub hours: i32,
    pub minutes: i32,
    pub hourly_data: Vec<f64>,
}

impl Default for TodayData {
    fn default() -> Self {
        Self {
            hours: 0,
            minutes: 0,
            hourly_data: vec![0.0; 12],
        }
    }
}

pub struct Statistics {
    pub today_data: TodayData,
}

impl Statistics {
    pub fn new(today_data: TodayData) -> Self {
        Self { today_data }
    }
}

impl Widget for Statistics {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::new();
        let inner = block.inner(area);
        block.render(area, buf);

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .split(inner);

        render_today_box(layout[0], buf, &self.today_data);

        let second = vec![Line::from("week").bold().green().centered()];
        Paragraph::new(second)
            .block(Block::bordered())
            .render(layout[1], buf);

        let third = vec![Line::from("avg").bold().green().centered()];
        Paragraph::new(third)
            .block(Block::bordered())
            .render(layout[2], buf);

        let fourth = vec![Line::from("streak").bold().green().centered()];
        Paragraph::new(fourth)
            .block(Block::bordered())
            .render(layout[3], buf);
    }
}

fn render_today_box(area: Rect, buf: &mut Buffer, data: &TodayData) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(139, 92, 246)));

    let inner = block.inner(area);
    block.render(area, buf);

    if inner.height < 3 {
        // TODO: need to handle this
        return;
    }

    let chunks = Layout::vertical([
        Constraint::Length(1), // Header
        Constraint::Length(1), // Time display
        Constraint::Length(1), // Bar graph (fixed height)
        Constraint::Min(0),    // Remaining space
    ])
    .split(inner);

    // Header
    let header = Line::from(vec![
        Span::styled("⚡ ", Style::default().fg(Color::Rgb(139, 92, 246))),
        Span::styled("TODAY", Style::default().fg(Color::Rgb(139, 92, 246))),
    ]);
    Paragraph::new(header).render(chunks[0], buf);

    // Time
    let time_text = format!("{}h  {}m", data.hours, data.minutes);
    let time = Line::from(time_text).style(
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    );
    Paragraph::new(time).render(chunks[1], buf);

    // Bar chart
    render_custom_bars(chunks[2], buf, &data.hourly_data);
}

fn render_custom_bars(area: Rect, buf: &mut Buffer, hourly_data: &[f64]) {
    if hourly_data.is_empty() || area.width < 12 || area.height == 0 {
        return;
    }

    let max_seconds = hourly_data.iter().copied().fold(0.0_f64, f64::max);

    if max_seconds == 0.0 {
        return;
    }

    // Calculate how many characters wide each bar should be
    let total_bars = hourly_data.len().min(12);
    let chars_per_bar = (area.width as usize / total_bars).max(1);
    let bar_color = Color::Rgb(139, 92, 246);

    // Use block characters for bars
    let bar_chars = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

    for (i, &seconds) in hourly_data.iter().take(total_bars).enumerate() {
        if seconds == 0.0 {
            continue;
        }

        // Calculate bar height (0-8 levels based on bar_chars)
        let normalized = (seconds / max_seconds * 8.0).ceil() as usize;
        let bar_level = normalized.min(8).max(1) - 1;
        let bar_char = bar_chars[bar_level];

        // Calculate x position for this bar
        let x_start = area.x + (i * chars_per_bar) as u16;

        // Draw the bar across multiple characters for width
        for offset in 0..chars_per_bar {
            let x = x_start + offset as u16;
            if x < area.x + area.width {
                // Place bar on the single line (area.y since height is 1)
                let y = area.y;
                if let Some(cell) = buf.cell_mut((x, y)) {
                    cell.set_symbol(&bar_char.to_string());
                    cell.set_fg(bar_color);
                }
            }
        }
    }
}
