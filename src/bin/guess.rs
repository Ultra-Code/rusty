use rusty::guessing_game;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    guessing_game::start()?;
    Ok(())
}
