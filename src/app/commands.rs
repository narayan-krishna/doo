use super::{lists::Navigate, DooList, Mode, Screen, RecentFiles};

pub fn saveas(input: Option<&&str>, doolist: &mut DooList, recent_files: &mut RecentFiles, current_path: &Option<String>) {
    match input {
        Some(path) => {
            eprintln!("saving to {}", path);
            doolist.save(path.to_string()).unwrap();
            recent_files.add_recent(path.to_string());
        }
        None => {
            if let Some(path) = &current_path {
                eprintln!("saving to {}", path);
                doolist.save(path.to_string()).unwrap();
                recent_files.add_recent(path.to_string());
            }
        }
    }
}

// BUG: should propogate an error on load failure
pub fn load(input: Option<String>, doolist: &mut DooList, recent_files: &mut RecentFiles, current_path: &mut Option<String>) {
    if let Some(path) = input {
        match DooList::load(path.to_string()) {
            Ok(list) => {
                *doolist = list;
                *current_path = Some(path.to_string());
                recent_files.add_recent(path.to_string());
                doolist.next();
            }
            Err(e) => eprintln!("that was an error {}", e),
        }
    }
}

pub fn new(doolist: &mut DooList, current_path: &mut Option<String>) {
    *current_path = None;
    *doolist = DooList::new();
}

pub fn changename(input: Option<&&str>, doolist_name: &mut Option<String>) {
    if let Some(name) = input {
        *doolist_name = Some(name.to_string());
    }
}

pub fn help(screen: &mut Screen, mode: &mut Mode) {
    *screen = Screen::Help;
    *mode = Mode::Select;
}

pub fn recent(screen: &mut Screen, mode: &mut Mode) {
    *screen = Screen::Recents;
    *mode = Mode::Select;
}

pub fn quit(quit_state: &mut bool) {
    *quit_state = true;
}