use rusty::webserver;

pub fn main() -> std::io::Result<()> {
    webserver::server()?;
    Ok(())
}
