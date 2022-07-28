mod app;
mod err;
mod ui;

use std::{io, time::{Instant, Duration} };
use app::App;
use tui::{backend::{Backend, CrosstermBackend}, Terminal};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> io::Result<()> {
  let mut last_tick = Instant::now();
  loop {
    terminal.draw(|f| ui::draw(f, &mut app))?;
    let timeout = app.state.tick_rate.checked_sub(last_tick.elapsed().as_millis() as u64)
      .unwrap_or_else(|| 0);
    if crossterm::event::poll(Duration::from_millis(timeout))? {
      if let Event::Key(key) = event::read()? {
	app.on_key(key.code);
      }
      if last_tick.elapsed().as_millis() as u64 >= app.state.tick_rate {
        app.on_tick();
        last_tick = Instant::now();
      }
      if app.status == app::AppStatus::SHUTDOWN {
	return Ok(());
      }
    }
  }
}

pub fn run(tick_rate: u64) -> Result<(), io::Error> {
  // setup terminal
  enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  // setup app
  let app = App::new("client", true, tick_rate);

  // runit
  run_app(&mut terminal, app)?;

  // restore terminal
  disable_raw_mode()?;
  execute!(
    terminal.backend_mut(),
    LeaveAlternateScreen,
    DisableMouseCapture
  )?;
  terminal.show_cursor()?;
  
  Ok(())

}

fn main() -> Result<(), io::Error> {
  run(100)
}
