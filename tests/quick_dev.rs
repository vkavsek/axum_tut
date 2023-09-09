#![allow(unused)]

use anyhow::{Result};

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    // Sends requests to the server and prints the result 
    hc.do_get("/hello?name=Vid").await?.print().await?;
    hc.do_get("/hello2/Luka").await?.print().await?;

    Ok(())
}
