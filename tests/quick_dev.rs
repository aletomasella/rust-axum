#![allow(unused)]

use anyhow::{Ok, Result};

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3001")?;

    hc.do_get("/hello?name=TestingQuery").await?.print().await?;

    hc.do_get("/hello2/TestingParams").await?.print().await?;

    // hc.do_get("/src/main.rs").await?.print().await?;

    Ok(())
}
