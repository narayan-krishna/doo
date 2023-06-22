pub trait Navigate {
    fn previous(&mut self) -> ();
    fn next(&mut self) -> ();
}

pub trait Saveable {
    fn save() -> ();
}

pub trait Loadable {
    fn load() -> ();
}

pub trait Selectable {
    fn select() -> ();
}
