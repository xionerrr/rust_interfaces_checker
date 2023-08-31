use std::env;

use crate::utils::{find_shoe, Shoe};

mod app;
mod config;
mod models;
mod utils;

pub fn create_app() -> eyre::Result<()> {
    let config = utils::env_parser::parse_env::<config::AppConfig>()?;

    let current_path = env::current_dir()?;
    let current_path_str: &str = current_path.to_str().expect("Failed to parse");
    let directories = utils::directories_parser::parse_directories(current_path_str);

    println!("{:?}", config);
    println!("{:?}", directories);

    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = find_shoe::find_shoe(shoes, 13);

    println!("{:?}", in_my_size);

    Ok(())
}
