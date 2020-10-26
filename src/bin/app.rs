use std::error::Error;
use th_backend::server;

fn main() -> Result<(), Box<dyn Error>> {
    server::run();    

    Ok(())
}