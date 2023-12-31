use core::fmt::Display;
use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::Path;
use tui::widgets::ListState;

use super::lists;

#[derive(Serialize, Deserialize, Clone)]
pub struct DooItem {
    pub label: String,
    pub complete: bool,
}

impl DooItem {
    pub fn new(label: String, complete: bool) -> DooItem {
        DooItem { label, complete }
    }

    pub fn change_label(&mut self, label: String) {
        self.label = label
    }

    pub fn mark(&mut self) {
        self.complete = !self.complete
    }
}

#[derive(Serialize, Deserialize)]
pub struct DooList {
    pub name: Option<String>,
    pub list: Vec<DooItem>,
    #[serde(skip)]
    pub state: ListState,
    #[serde(skip)]
    pub path: Option<String>,
}

impl DooList {
    pub fn new() -> DooList {
        DooList {
            name: None,
            list: Vec::new(),
            state: ListState::default(),
            path: None,
        }
    }

    // TODO: impl error handling for this function
    pub fn load(path: &String) -> Result<DooList, &'static str> {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(_) => return Err("failed to find/open file"), //TODO : propogate
        };

        let reader = BufReader::new(file);

        match serde_json::from_reader(reader) {
            Ok(list) => return Ok(list),
            Err(_) => return Err("failed to get list from file"), //TODO : propogate
        }
    }

    pub fn save(&self, path: &String) -> std::io::Result<()> {
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

    pub fn change_name(&mut self, new_name: String) {
        self.name = Some(new_name);
    }

    pub fn add_from_label(&mut self, label: String) {
        self.list.push(DooItem {
            label,
            complete: false,
        });

        self.select_last();
    }

    pub fn add_from_item(&mut self, item: DooItem) {
        self.list.push(item);
        self.select_last()
    }

    fn select_last(&mut self) {
        self.state.select(Some(self.list.len() - 1));
    }

    pub fn remove(&mut self) -> Option<DooItem> {
        match self.list.len() == 0 {
            true => None,
            false => {
                if let Some(i) = self.state.selected() {
                    let removed_item: DooItem = self.list[i].clone();
                    if self.list.len() == 1 {
                        self.state = ListState::default();
                    } else if self.list.len() - 1 == i {
                        self.state.select(Some(i - 1));
                    }
                    self.list.remove(i);
                    return Some(removed_item);
                }

                return None;
            }
        }
    }

    pub fn mark_selection(&mut self) -> Result<(), &'static str> {
        match self.state.selected() {
            None => Err("no selection to mark as complete"),
            Some(i) => Ok(self.list[i].mark()),
        }
    }

    pub fn change_label_name(&mut self, label: String) -> Result<(), &'static str> {
        match self.state.selected() {
            None => Err("no selection to change label name"),
            Some(i) => Ok(self.list[i].change_label(label)),
        }
    }

    pub fn add_subtodo() {
        todo!()
    }
}

impl lists::Navigate for DooList {
    fn previous(&mut self) {
        if self.list.len() != 0 {
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
        if self.list.len() != 0 {
            let i = match self.state.selected() {
                Some(i) => match i >= self.list.len() - 1 {
                    true => i,
                    false => i + 1,
                },
                None => 0,
            };
            self.state.select(Some(i));
        }
    }
}

#[cfg(test)]
mod tests {}
