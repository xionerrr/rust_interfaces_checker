#![allow(unused)]

use rust_scrn;

fn main() -> eyre::Result<()> {
    rust_scrn::create_app()?;
    Ok(())
}
