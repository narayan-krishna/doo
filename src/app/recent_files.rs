use super::{ lists::Navigate, queue::CappedQueue };
use tui::widgets::ListState;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct RecentFiles {
    pub queue: CappedQueue<String>,
    #[serde(skip)]
    pub state: ListState,
}

impl RecentFiles {
    pub fn new(capacity: Option<usize>) -> RecentFiles {
        RecentFiles {
            queue: CappedQueue::new(match capacity {
                Some(i) => i,
                None => 5,
            }),
            state: ListState::default(),
        }
    }

    // TODO: impl error handling for this function
    pub fn load(path: &str) -> RecentFiles {
        let file = File::open(path).expect("failed to open file");
        let reader = BufReader::new(file);
        let list: serde_json::Result<RecentFiles> = serde_json::from_reader(reader);

        if let Ok(mut recent_files) = list {
            if recent_files.queue.items.len() > 0 {
                recent_files.next();
            }
            return recent_files;
        } else {
            eprintln!("failed to load RecentFiles file");
            return RecentFiles::new(Some(5));
        }
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        eprintln!("attempting to save...");

        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)
            .expect("unable to open file");

        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self)?;

        Ok(())
    }

    pub fn select(&mut self) -> Option<String> {
        let mut ret = None;

        if let Some(i) = self.state.selected() {
            match self.queue.items.get(i) {
                Some(path) => { ret = Some(path.to_string()); },
                None => {
                    eprintln!("failed to get item from queue");
                    return None;
                }
            };
        }

        let ret_clone = ret.clone();
        if let Some(path) = ret_clone {
            self.add_recent(path)
        }

        return ret;
    }

    pub fn add_recent(&mut self, path: String) {
        // find if item is filepath exists in queue
        let mut curr_index: Option<usize> = None;
        for (i, a) in self.queue.items.iter().enumerate() {
            if *a == path {
                curr_index = Some(i);
                break;
            }
        }

        // if yes, delete 
        if let Some(i) = curr_index {
            self.queue.items.remove(i);
        }

        // if at capacity, pop back
        if self.queue.is_full() {
            self.queue.pop_back();
        }

        // push to front
        if let Err(e) = self.queue.push_front(path) {
            eprintln!("{}", e);
        }
    }
}

impl Navigate for RecentFiles {
    fn previous(&mut self) {
        if self.queue.items.len() != 0 {
            let i = match self.state.selected() {
                Some(i) => match i == 0 {
                    true => i,
                    false => i - 1,
                },
                None => 0,
            };
            self.state.select(Some(i));
        }
    }

    fn next(&mut self) {
        if self.queue.items.len() != 0 {
            let i = match self.state.selected() {
                Some(i) => match i >= self.queue.items.len() - 1 {
                    true => i,
                    false => i + 1,
                },
                None => 0,
            };
            self.state.select(Some(i));
        }
    }
}
