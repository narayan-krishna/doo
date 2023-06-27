pub mod app;

pub fn run(filepath: Option<String>) {
    let app = app::App::new(filepath);
    app::run(app).unwrap();
}

pub mod utils {
    use path_clean::PathClean;
    use std::env;
    use std::path::Path;

    pub fn get_abs_path_from(local_path_str: String) -> String {
        let path = Path::new(&local_path_str);
        let absolute_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            env::current_dir().unwrap().join(path)
        }
        .clean();

        // TODO: error handling
        absolute_path.into_os_string().into_string().unwrap()
    }
}
