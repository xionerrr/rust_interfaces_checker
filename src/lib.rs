use app::App;

mod app;
mod config;
mod env_parser;

pub struct Parameters {}

pub fn create_app() -> eyre::Result<App> {
    let config = env_parser::parse_env::<config::AppConfig>()?;
    let app: App = App::new();
    app.inject_dep().run();
}
