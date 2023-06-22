#![allow(unused_imports)]

pub mod doolist;
mod queues;
mod ui;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use doolist::{DooItem, DooList};
use queues::UndoQueue;
use std::{
    collections::VecDeque,
    error, io,
    time::{Duration, Instant},
};
use tui::{backend::CrosstermBackend, Terminal};

pub enum Mode {
    Search,
    Select,
    Command,
    Input,
}

// TODO: look into error logging
pub struct App {
    doolist: DooList,
    quit_state: bool,
    input: String,
    mode: Mode,
    current_path: Option<String>,
    undo_delete_queue: UndoQueue<DooItem>,
}

impl App {
    pub fn new(filepath: Option<String>) -> App {
        // app needs its own config file, in addition todo files
        let mut app = App {
            doolist: DooList::new(),
            quit_state: false,
            input: String::from(""),
            mode: Mode::Select,
            current_path: None,
            undo_delete_queue: UndoQueue::new(5),
        };

        let valid_filepath: Option<String> = if let Some(path) = filepath {
            Some(path)
        } else if let Ok(path) = Self::most_recent_save() {
            Some(path)
        } else {
            None
        };

        if let Some(path) = valid_filepath {
            // try to load the most recent file
            eprintln!("attempting to load path into doolist");
            let saved_path: String = path.clone();
            match DooList::load(path) {
                Ok(list) => {
                    app.doolist = list;
                    app.current_path = Some(saved_path);
                }
                Err(e) => eprintln!("{}", e),
            }
        }

        app
    }

    fn most_recent_save() -> Result<String, Box<dyn error::Error>> {
        return Ok(String::from("/home/knara/dev/rust/doo/test.json"));
        // return Err("there is no most recent file path".into());
    }

    #[inline]
    pub fn handle_select(&mut self, key_code: crossterm::event::KeyCode) {
        match key_code {
            KeyCode::Char('q') => self.quit_state = true,
            KeyCode::Char('a') => {
                self.doolist.add_from_label(String::from("-- new task --"));
                self.mode = Mode::Input;
            }
            KeyCode::Char('j') => self.doolist.next(),
            KeyCode::Char('k') => self.doolist.previous(),
            KeyCode::Char('x') => {
                self.doolist.mark_selection(); // TODO: this should eventually print to an error
                                               // message widget
                return;
            }
            KeyCode::Char('d') => {
                let deleted_item: Option<DooItem> = self.doolist.remove();
                if let Some(item) = deleted_item {
                    self.undo_delete_queue.push(item);
                };
            }
            KeyCode::Char('u') => match self.undo_delete_queue.pop() {
                Some(item) => self.doolist.add_from_item(item),
                None => {}
            },
            KeyCode::Char('i') => self.mode = Mode::Input,
            KeyCode::Char(':') => self.mode = Mode::Command,
            _ => {}
        }
    }

    #[inline]
    pub fn handle_search(&mut self, _key_code: crossterm::event::KeyCode) {
        todo!()
    }

    #[inline]
    pub fn handle_command(&mut self, key_code: crossterm::event::KeyCode) {
        match key_code {
            KeyCode::Enter => {
                self.mode = Mode::Select;
                self.run_input_command(self.input.clone());
                self.input.clear();
            }
            KeyCode::Esc => {
                self.mode = Mode::Select;
                self.input.clear();
            }
            KeyCode::Char(c) => {
                self.input.push(c);
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            _ => {}
        }
    }

    #[inline]
    pub fn handle_input(&mut self, key_code: crossterm::event::KeyCode) {
        match key_code {
            KeyCode::Enter => {
                self.mode = Mode::Select;
                if let Err(e) = self.doolist.change_label_name(self.input.clone()) {
                    eprintln!("{}", e);
                }
                self.input.clear();
            }
            KeyCode::Esc => {
                self.mode = Mode::Select;
                self.input.clear();
            }
            KeyCode::Char(c) => {
                self.input.push(c);
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            _ => {}
        }
    }

    fn run_input_command(&mut self, input: String) {
        let elements: Vec<&str> = input.split(' ').collect();
        match elements[0] {
            "saveas" | "w" => match elements.get(1) {
                Some(path) => {
                    self.doolist.saveas(path.to_string()).unwrap();
                }
                None => {
                    if let Some(path) = &self.current_path {
                        self.doolist.saveas(path.to_string()).unwrap();
                    }
                }
            },
            "load" => match DooList::load(elements[1].to_string()) {
                Ok(list) => {
                    self.doolist = list;
                    self.current_path = Some(elements[1].to_string());
                }
                Err(e) => eprintln!("that was an error {}", e),
            },
            "changename" => if let Some(name) = elements.get(1) {
                self.doolist.name = Some(name.to_string());
            },
            "recents" => todo!(),
            "q" => self.quit_state = true,
            _ => {}
        }
    }
}

pub fn run(mut app: App) -> Result<(), io::Error> {
    // do stuff with list
    enable_raw_mode()?;
    let mut stdout = io::stderr(); // change this from stdout to stderr
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;
    let tick_rate = Duration::from_millis(50);

    let mut last_tick = Instant::now();
    loop {
        term.draw(|f| ui::draw(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match app.mode {
                    Mode::Select => app.handle_select(key.code),
                    Mode::Search => app.handle_search(key.code),
                    Mode::Command => app.handle_command(key.code),
                    Mode::Input => app.handle_input(key.code),
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
        if app.quit_state {
            break;
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    term.show_cursor()?;

    Ok(())
}
