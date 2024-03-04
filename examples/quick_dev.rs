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
            "id": 2,
            "method": "list_tasks",
        }),
    );

    let req_update_task = hc.do_post(
        "/api/rpc",
        json!({
            "id": 3,
            "method": "update_task",
            "params": {
                "id": 1000,
                "data": {
                    "title": "task AAA updated"
                }
            }
        }),
    );

    let req_delete_task = hc.do_post(
        "/api/rpc",
        json!({
            "id": 4,
            "method": "delete_task",
            "params": {
                "id": 1001,
            }
        }),
    );

    let req_list_tasks_2 = hc.do_post(
        "/api/rpc",
        json!({
            "id": 5,
            "method": "list_tasks",
        }),
    );

    req_login.await?.print().await?;
    req_create_task.await?.print().await?;
    req_list_tasks.await?.print().await?;
    req_update_task.await?.print().await?;
    req_delete_task.await?.print().await?;
    req_list_tasks_2.await?.print().await?;
    req_logoff.await?.print().await?;

    Ok(())
}
