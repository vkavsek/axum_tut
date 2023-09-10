#![allow(unused)]

use anyhow::{Result};
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    // Sends requests to the server and prints the result 
    hc.do_get("/hello2/Luka San").await?.print().await?;

    let req_login = hc.do_post("/api/login", 
        json!({
            "uname": "demo1", 
            "pass": "1234"
        })
    );

    req_login.await?.print().await?;

    Ok(())
}
