use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph},
};

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
    let block = Block::bordered();
    let inner = block.inner(area);

    frame.render_widget(block, area);

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(inner);

    let left = vec![Line::from("name").bold().green().left_aligned()];
    let right = vec![Line::from("updated").bold().green().right_aligned()];

    frame.render_widget(Paragraph::new(left), layout[0]);
    frame.render_widget(Paragraph::new(right), layout[1])
}

pub fn render_statistics(frame: &mut Frame, area: Rect) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(area);

    let first = vec![Line::from("today").bold().green().centered()];
    let second = vec![Line::from("week").bold().green().centered()];
    let third = vec![Line::from("avg").bold().green().centered()];
    let fourth = vec![Line::from("streak").bold().green().centered()];

    frame.render_widget(
        Paragraph::new(first).block(Block::bordered()).centered(),
        layout[0],
    );

    frame.render_widget(
        Paragraph::new(second).block(Block::bordered()).centered(),
        layout[1],
    );

    frame.render_widget(
        Paragraph::new(third).block(Block::bordered()).centered(),
        layout[2],
    );

    frame.render_widget(
        Paragraph::new(fourth).block(Block::bordered()).centered(),
        layout[3],
    )
}

pub fn render_projects(frame: &mut Frame, area: Rect) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);

    let first = vec![Line::from("languages").bold().green().centered()];
    let second = vec![Line::from("projects").bold().green().centered()];

    frame.render_widget(
        Paragraph::new(first).block(Block::bordered()).centered(),
        layout[0],
    );

    frame.render_widget(
        Paragraph::new(second).block(Block::bordered()).centered(),
        layout[1],
    );
}

pub fn render_today(frame: &mut Frame, area: Rect) {
    let text = vec![Line::from("today").bold().green().left_aligned()];

    frame.render_widget(
        Paragraph::new(text).block(Block::bordered()).centered(),
        area,
    )
}

pub fn render_footer(frame: &mut Frame, area: Rect) {
    let text = vec![Line::from("footer").bold().green().left_aligned()];

    frame.render_widget(
        Paragraph::new(text).block(Block::bordered()).centered(),
        area,
    )
}
