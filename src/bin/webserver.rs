use book::webserver;

pub fn main() -> std::io::Result<()> {
    webserver::server()?;
    Ok(())
}
