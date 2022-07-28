use crossterm::event::KeyCode;
use tui::widgets::TableState;

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AppStatus {
  CONTINUE,
  SHUTDOWN,
}

pub enum InputMode {
  Normal,
  Editing,
}

pub struct AppState<'a> {
  pub tabs: TabsState<'a>,
  pub tables: Vec<TableState>,
  pub input: String,
  pub input_mode: InputMode,
  pub input_history: Vec<String>,
  pub boost: bool,
  pub tick_rate: u64,
}

pub struct AppData<'a> {
  pub server: &'a str,
  pub agents: Vec<Vec<&'a str>>,
}

pub struct App<'a> {
  pub title: &'a str,
  pub status: AppStatus,
  pub state: AppState<'a>,
  pub data: AppData<'a>,
}

impl<'a> App<'a> {
  pub fn new(title: &'a str, boost: bool, tick_rate: u64) -> App<'a> {
    let status = AppStatus::CONTINUE;
    let tabs = TabsState::new(vec!["cmd", "1", "2"]);
    let input = String::new();
    let input_mode = InputMode::Normal;
    let input_history = vec![];
    let tables = vec![TableState::default()];
    let data = AppData { server: "127.0.0.1", agents: vec!["agent0", "agent1"] };
    let state = AppState { tabs, tables, input, input_mode, input_history, boost, tick_rate};

    App {
      title,
      status,
      state,
      data,
    }
  }

  pub fn on_key(&mut self, key_code: KeyCode) {
    match self.state.input_mode {
      InputMode::Normal => self.on_key_normal(key_code),
      InputMode::Editing => self.on_key_editing(key_code),
    }
  }

  pub fn on_key_normal(&mut self, key_code: KeyCode) {
    match key_code {
      KeyCode::Char(c) => match c {
	'q' => {

	  self.status = AppStatus::SHUTDOWN;
	},
	'0' => {
	  self.state.tabs.index = 0;
	},
	'1' => {
	  self.state.tabs.index = 1;
	},
	'2' => {
	  self.state.tabs.index = 2;
	},
	':' => {
	  self.state.input_mode = InputMode::Editing;
	},
	'`' => {
	  self.state.tabs.index = 0;
	},
	_ => {}
      },
      KeyCode::Left => {
	self.state.tabs.previous();
      },
      KeyCode::Right => {
	self.state.tabs.next();
      },
      KeyCode::Tab => {
	self.state.tabs.next();
      },
      _ => {}
    }
    
  }

  pub fn on_key_editing(&mut self, key_code: KeyCode) {
    match key_code {
      KeyCode::Enter => {
	self.state.input_history.push(self.state.input.drain(..).collect());
      },
      KeyCode::Char(c) => {
	self.state.input.push(c);
      },
      KeyCode::Backspace => {
	self.state.input.pop();
      },
      KeyCode::Esc => {
	self.state.input_mode = InputMode::Normal;
      },
      _ => {}
    }
  }

  pub fn on_tick(&mut self) {

  }
}
