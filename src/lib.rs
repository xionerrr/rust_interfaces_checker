use std::env;

mod config;
mod directories_parser;
mod env_parser;

pub fn create_app() -> eyre::Result<()> {
    let config = env_parser::parse_env::<config::AppConfig>()?;

    let current_path = env::current_dir()?;
    let current_path_str = current_path.to_str().expect("Failed to parse");
    let directories = directories_parser::parse_directories(current_path_str);

    println!("{:?}", config);
    println!("{:?}", directories);

    Ok(())
}
