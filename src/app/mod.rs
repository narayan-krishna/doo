#![allow(unused_imports)]

mod commands;
pub mod doolist;
mod lists;
mod queue;
mod recent_files;
mod ui;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use super::{ utils, config };
use doolist::{DooItem, DooList};
use lists::*;
use queue::CappedQueue;
use recent_files::RecentFiles;
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

pub enum Screen {
    DooList,
    Help,
    Recents,
}

const RECENT_FILES_PATH: &str = "/home/knara/dev/rust/doo/src/recent_files.json";

// TODO: look into error logging
pub struct App {
    config: config::DooConfig,
    screen: Screen,
    mode: Mode,
    doolist: DooList,
    recent_files: RecentFiles,
    current_path: Option<String>,
    undo_delete_queue: CappedQueue<DooItem>,
    quit_state: bool,
    input: String,
}

impl App {
    pub fn new(filepath: Option<String>, config: config::DooConfig) -> App {
        // app needs its own config file, in addition todo files
        let mut app = App {
            config,
            screen: Screen::DooList,
            mode: Mode::Select,
            input: String::from(""),
            doolist: DooList::new(),
            recent_files: RecentFiles::load(RECENT_FILES_PATH),
            current_path: None,
            undo_delete_queue: CappedQueue::new(5),
            quit_state: false,
        };

        let valid_filepath: Option<String> = if let Some(path) = filepath {
            Some(utils::get_abs_path_from(path))
        } else if let Ok(path) = app.most_recent_save() {
            Some(path)
        } else {
            None
        };

        commands::load(
            valid_filepath,
            &mut app.doolist,
            &mut app.recent_files,
            &mut app.current_path,
        );
        app
    }

    fn most_recent_save(&mut self) -> Result<String, Box<dyn error::Error>> {
        if let Some(i) = self.recent_files.queue.items.get(0) {
            return Ok(i.to_string());
        }

        return Err("there is no most recent file path".into());
    }

    pub fn handle_quit(&mut self) {
        self.recent_files.save(RECENT_FILES_PATH).unwrap();
    }

    #[inline]
    pub fn handle_select(&mut self, key_code: crossterm::event::KeyCode) {
        match self.screen {
            Screen::DooList => match key_code {
                KeyCode::Char('q') => self.quit_state = true,
                KeyCode::Char('a') => {
                    self.doolist.add_from_label(String::from("-- new task --"));
                    self.undo_delete_queue.clear().unwrap();
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
                        self.undo_delete_queue.push_front(item);
                    };
                }
                KeyCode::Char('u') => match self.undo_delete_queue.pop_front() {
                    Some(item) => self.doolist.add_from_item(item),
                    None => {}
                },
                KeyCode::Char('i') => self.mode = Mode::Input,
                KeyCode::Char(':') => self.mode = Mode::Command,
                _ => {}
            },
            Screen::Help => match key_code {
                KeyCode::Esc => self.screen = Screen::DooList,
                _ => {}
            },
            Screen::Recents => match key_code {
                KeyCode::Char('j') => self.recent_files.next(),
                KeyCode::Char('k') => self.recent_files.previous(),
                KeyCode::Enter => {
                    let selected_path = self.recent_files.select();
                    commands::load(
                        selected_path,
                        &mut self.doolist,
                        &mut self.recent_files,
                        &mut self.current_path,
                    );
                    // BUG: should only return to doolist on load success
                    self.screen = Screen::DooList;
                }
                KeyCode::Esc => self.screen = Screen::DooList,
                _ => {}
            },
        }
    }

    #[inline]
    pub fn handle_search(&mut self, _key_code: crossterm::event::KeyCode) {
        todo!()
    }

    #[inline]
    pub fn handle_command_input(&mut self, key_code: crossterm::event::KeyCode) {
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
    pub fn handle_label_input(&mut self, key_code: crossterm::event::KeyCode) {
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
            "save" => commands::saveas(
                None,
                &mut self.doolist,
                &mut self.recent_files,
                &self.current_path,
            ),
            "saveas" | "w" => commands::saveas(
                match elements.get(1) {
                    Some(i) => Some(utils::get_abs_path_from(i.to_string())),
                    None => None,
                },
                &mut self.doolist,
                &mut self.recent_files,
                &self.current_path,
            ),
            "load" | "e" => match elements.get(1) {
                Some(i) => commands::load(
                    Some(utils::get_abs_path_from(i.to_string())),
                    &mut self.doolist,
                    &mut self.recent_files,
                    &mut self.current_path,
                ),
                None => eprintln!("failed to load file with supplied path"),
            },
            "wq" => {
                commands::saveas(
                    None,
                    &mut self.doolist,
                    &mut self.recent_files,
                    &self.current_path,
                );
                commands::quit(&mut self.quit_state);
            }
            "new" => commands::new(&mut self.doolist, &mut self.current_path),
            "rename" => commands::rename(elements.get(1), &mut self.doolist.name),
            "help" => commands::help(&mut self.screen, &mut self.mode),
            "recent" => commands::recent(&mut self.screen, &mut self.mode),
            "path" => eprintln!(
                "{}",
                if let Some(i) = &self.current_path {
                    i
                } else {
                    "xd"
                }
            ),
            "q" => commands::quit(&mut self.quit_state),
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
                    Mode::Command => app.handle_command_input(key.code),
                    Mode::Input => app.handle_label_input(key.code),
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
        if app.quit_state {
            app.handle_quit();
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
