use anyhow::Result;
use serde_json::json;

// Quote from https://www.youtube.com/watch?v=XZtlD_m59sM
// On *Windows* for the "link" file issue when running the cargo test and run in parallel. The solution is to move `tests/quick_dev.rs` to the `examples` folder, rename the function to `#[tokio::main]`, and it should allow you to do the following:
// - In Terminal 1: `cargo watch -q -c -w src/ -x run`
// - In Terminal 2: `cargo watch -q -c -w examples/ -x 'run --example quick_dev'`
//

// Because Wins has problem with run tests and main parallel
// Need to move webserver run test to examples
#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello?name=Alex").await?.print().await?;
    hc.do_get("/hello/Alex").await?.print().await?;
    // Fallback
    // hc.do_get("/src/main.rs").await?.print().await?;
    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "client",
            "pwd": "123456",
        }),
    );
    req_login.await?.print().await?;
    hc.do_get("/hello?name=John").await?.print().await?;

    hc.do_post(
        "/api/tickets",
        json!({
            "name": "Failed Ticket",
        }),
    )
    .await?
    .print()
    .await?;

    hc.do_get("/api/tickets").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "admin",
            "pwd": "123456",
        }),
    );
    req_login.await?.print().await?;

    hc.do_post(
        "/api/tickets",
        json!({
            "name": "Success Ticket",
        }),
    )
    .await?
    .print()
    .await?;

    hc.do_get("/api/tickets").await?.print().await?;
    hc.do_delete("/api/tickets/1").await?.print().await?;
    hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}
