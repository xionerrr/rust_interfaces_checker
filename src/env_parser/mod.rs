use config::Config;
use serde::Deserialize;

pub fn parse_env<'a, T: Deserialize<'a>>() -> eyre::Result<T> {
    let config = Config::builder()
        .add_source(config::Environment::default())
        .build()?;

    let data: T = config.try_deserialize()?;

    Ok(data)
}
