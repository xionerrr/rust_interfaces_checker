use rust_scrn::Application;

pub fn main() -> eyre::Result<()> {
    let app = rust_scrn::create_app()?;
    app.run()?;
    Ok(())
}
