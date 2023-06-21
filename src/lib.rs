pub mod app;

pub fn run(filepath: Option<String>) {
    let app = app::App::new(filepath);
    app::run(app).unwrap();
}
