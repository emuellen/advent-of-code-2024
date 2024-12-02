use std::error::Error;

mod day1;
mod day2;

fn main() -> Result<(), Box<dyn Error>> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }

    env_logger::init();

    day1::run()?;
    day2::run()?;
    Ok(())
}
