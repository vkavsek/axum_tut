#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        }),
    );
    let req_logoff = hc.do_post(
        "/api/logoff",
        json!({
            "logoff": true,
        }),
    );

    let req_create_task = hc.do_post(
        "/api/rpc",
        json!({
            "id": 1,
            "method": "create_task",
            "params": {
                "data": {
                    "title": "task AAA"
                }
            }
        }),
    );

    let req_list_tasks = hc.do_post(
        "/api/rpc",
        json!({
            "id": 3,
            "method": "list_tasks",
        }),
    );

    req_login.await?.print().await?;
    req_create_task.await?.print().await?;
    req_list_tasks.await?.print().await?;
    req_logoff.await?.print().await?;

    Ok(())
}
