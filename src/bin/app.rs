use th_backend::models::{ competition::Competition, base_model::Model };
use std::error::Error;
use async_std;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _b = Competition::find_many(None, None).await;

    Ok(())
}