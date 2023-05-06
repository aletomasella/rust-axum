#![allow(unused)]

use anyhow::{Result, Ok};


#[tokio::test]
async fn quick_dev () -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3001")?;

  hc.do_get("/hello").await?.print().await?;

    Ok(())
}