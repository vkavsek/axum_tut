#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    // Sends requests to the server and prints the result
    hc.do_get("/hello2/Luka San").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "uname": "demo1",
            "pass": "1234"
        }),
    );
    let create_ticket = hc.do_post("/api/tickets", 
        json!({
            "title": "successful_ticket"
        })
    );
    let create_fail_ticket = hc.do_post("/api/tickets", 
        json!({"title": ""}));

    let delete_ticket = hc.do_delete("/api/tickets/0");



    req_login.await?.print().await?;

    create_ticket.await?.print().await?;
    //create_fail_ticket.await?.print().await?;
    delete_ticket.await?.print().await?;
    //hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}
