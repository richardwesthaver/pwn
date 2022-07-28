use crate::app::{App, InputMode};
use unicode_width::UnicodeWidthStr;
use tui::{
  backend::Backend, Frame,
  layout::{Constraint, Direction, Layout, Rect},
  text::{Span, Spans, Text},
  style::{Style, Modifier, Color},
  widgets::{Block, Borders, Cell, Row, List, ListItem, Paragraph, Tabs, Wrap, Table, TableState},
};

pub fn draw<B: Backend>(f: &mut Frame<B>, mut app: &mut App) {
  draw_tabs(f, &mut app);
}

pub fn draw_cmd<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
    .split(area);

  let (input, style) = match app.state.input_mode {
    InputMode::Normal => (
      vec![
	Span::raw("Press "),
	Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
	Span::raw(" to exit, "),
        Span::styled(":", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to start editing.")
      ],
      Style::default().add_modifier(Modifier::RAPID_BLINK),
    ),
    InputMode::Editing => (
      vec![
        Span::raw("Press "),
        Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to stop editing, "),
        Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to record the message"),
      ],
      Style::default(),
    ),
  };
  let input = Paragraph::new(app.state.input.as_ref())
    .style(match app.state.input_mode {
      InputMode::Normal => Style::default(),
      InputMode::Editing => Style::default().fg(Color::Yellow),
    })
    .wrap(Wrap { trim: true })
    .block(Block::default().borders(Borders::ALL).title("cmd"));
  f.render_widget(input, chunks[0]);
  match app.state.input_mode {
    InputMode::Normal => {},
    InputMode::Editing => {
      f.set_cursor(
	chunks[0].x + app.state.input.width() as u16 + 1,
	chunks[0].y + 1,
      )
    }
  }
    let output: Vec<ListItem> = app.state.input_history
      .iter()
      .enumerate()
      .map(|(i, m)| {
	let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
	ListItem::new(content)
      })
      .collect();
}

pub fn draw_table0<B: Backend>(f: &mut Frame<B>, app: &mut App) {
  let state = TableState::default();
    let rects = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(5)
        .split(f.size());

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::Blue);
    let header_cells = ["ip", "status", "state", "jobs", "last_seen"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);
    let rows = app.data.agents.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item.iter().map(|c| Cell::from(*c));
        Row::new(cells).height(height as u16).bottom_margin(1)
    });
    let t = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("agents"))
        .highlight_style(selected_style)
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(50),
            Constraint::Length(30),
            Constraint::Min(10),
        ]);
    f.render_stateful_widget(t, rects[0], &mut app.state.tables[0]);
}

pub fn draw_tabs<B: Backend>(f: &mut Frame<B>, app: &mut App) {
  let chunks = Layout::default()
    .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
    .split(f.size());
  let titles = app
    .state
    .tabs
    .titles
    .iter()
    .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Green))))
    .collect();

  let tabs = Tabs::new(titles)
    .block(Block::default().borders(Borders::ALL).title(app.title))
    .highlight_style(Style::default().fg(Color::Yellow))
    .select(app.state.tabs.index);

  f.render_widget(tabs, chunks[0]);

  match app.state.tabs.index {
    0 => draw_tab0(f, app, chunks[1]),
    1 => draw_tab1(f, app, chunks[1]),
    2 => draw_tab2(f, app, chunks[1]),
    _ => {}
  };  
}

pub fn draw_tab0<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
  draw_cmd(f, app, area);
}

pub fn draw_tab1<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
  draw_table0(f, app);
}

pub fn draw_tab2<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {

}
